#!/usr/bin/env -S npx tsx --resolveJsonModule

import * as path from "node:path";
import { createWriteStream, promises as fs } from "node:fs";
import { promises as stream } from "node:stream";
import * as tar from "tar";

import { tchapConfig } from "../package.json";
import { exec as execCallback, spawn } from "node:child_process";
import { promisify } from "node:util";

interface TchapConfig {
  use_github: boolean;
  prod: TchapConfigEnv;
  dev: TchapConfigEnv;
}

interface TchapConfigEnv {
  "tchap-web_version": string;
  "tchap-web_archive_name": string;
  "tchap-web_github": { branch: string; repo: string; };
}

const config: TchapConfig = tchapConfig as TchapConfig;

if (!config) {
  console.error("No config found");
  process.exit(1);
}

// get argument env --env prod or dev
const TCHAP_ENV = process.argv[2] as keyof TchapConfig; // dev or prod;
console.log("TCHAP_ENV", TCHAP_ENV);
if (TCHAP_ENV !== "prod" && TCHAP_ENV !== "dev") {
  console.error("Invalid environment. Use either 'prod' or 'dev' as argument");
  process.exit(1);
}


const exec = promisify(execCallback);

const PACKAGE_URL_PREFIX = "https://github.com/tchapgouv/tchap-web-v4/releases/download/"
let SRC_DIR = "src";
let ARCHIVE_DIR = "archives";

async function downloadToFile(url: string, filename: string): Promise<void> {
    console.log("Downloading " + url + "...");

    try {
        const resp: Response = await fetch(url);
        if (!resp.ok) throw new Error(`unexpected response ${resp.statusText}`);
        if (!resp.body) throw new Error(`unexpected response has no body ${resp.statusText}`);
        console.log("resp.body", resp.body);

        await stream.pipeline(resp.body as any, createWriteStream(filename));
    } catch (e) {
        console.error(e);
        try {
            await fs.unlink(filename);
        } catch {}
        throw e;
    }
}

// Function to clone GitHub repository
async function cloneGitHubRepo(repoUrl: string, branch: string, targetDir: string): Promise<void> {
    console.log(`Cloning repository ${repoUrl} branch ${branch} to ${targetDir}...`);
    
    // Remove target directory if it exists
    try {
        await fs.rm(targetDir, { recursive: true, force: true });
        console.log("existing directory removed");
    } catch (e) {
        // Directory might not exist, that's fine
    }
    
    // Clone the specific branch
    await exec(`git clone --depth 1 --branch ${branch} ${repoUrl} ${targetDir}`);
    console.log(`Repository cloned successfully to ${targetDir}`);
}

async function buildFromLocalRepo(targetDir: string) {
  console.log(`----------------  Building from local repo ${targetDir}`);
  console.log("---------------- Install dependencies");
  const { stdout: installOut, stderr: installErr } = await exec(
    `yarn install --frozen-lockfile --no-cache`,
    { cwd: targetDir }
  );
  console.log(installOut);
  console.log(installErr);
  // Install dependencies without cache
  console.log(`---------------- Building tchap web`, targetDir);
  await exec(`yarn build`, { cwd: targetDir });

  console.log("----------------  Build completed successfully");

  console.log("----------------  Copying webapp dist folder to src folder");
  // copy the dist folder to the src folder
  await fs.cp(path.join(targetDir, "webapp/"), SRC_DIR, { recursive: true });
}

//  copy config.json depending on the environment
async function renameConfig(targetDir: string) {
    // env taken from package.json
    console.log(`Renaming config files by environment`, TCHAP_ENV );

    const prodConfigPath = path.join(targetDir, "config.prod.json");
    const preprodConfigPath = path.join(targetDir, "config.preprod.json");
    const devConfigPath = path.join(targetDir, "config.dev.json");
    const destConfigPath = path.join(targetDir, "config.json");

    const configObj: Record<string, string> = {
        "prod": prodConfigPath,
        "preprod": preprodConfigPath,
        "dev": devConfigPath
    }

    if (configObj[TCHAP_ENV]) {
      await fs.rename(configObj[TCHAP_ENV], destConfigPath);
    } else {
      console.log(
        "No env var found or incorrect. Should be prod, preprod or dev. Using prod as default."
      );
      await fs.rename(prodConfigPath, destConfigPath);
    }
}

async function buildFromGithubRepo(repoUrl: string, branch: string) {
    try {
        console.log("Building from github repo", branch);
        const targetDir = ARCHIVE_DIR + "/" + branch;
        await cloneGitHubRepo(repoUrl, branch, targetDir);
        await renameConfig(targetDir);
        await buildFromLocalRepo(targetDir);
    } catch(e) {
        console.log("Failed to build from github repo", e);
        return 1;
    }
}

async function buildFromArchive(targetVersion: string, filename: string) {
    let url: string | undefined =
        PACKAGE_URL_PREFIX + `tchap-${targetVersion}` + "/" + filename;
    let haveArchive = false;

    const selectedArchivePath = path.join(ARCHIVE_DIR, filename);
        console.log("Building from archive", targetVersion, filename);
    // check if we already downloaded the archive yet
    try {
        await fs.stat(selectedArchivePath);
        console.log(selectedArchivePath + " already exists");
        // means we don't need to download it again
        haveArchive = true;
    } catch (e: any) {
        console.log("the archive does not exist, proceed to download it");
    }

    try {
        if (!haveArchive) {
            console.log("downling archive ");
            await downloadToFile(url, selectedArchivePath);
        }
    } catch (e) {
        console.log("Failed to download " + url, e);
        return 1;
    }

    try {
        // we extract the downloaded file to the src folder
        // tar will overwrite the existing files and folder
        console.log(`Extracting the archives to ${SRC_DIR}`);
        await tar.x(
            {
                file: selectedArchivePath,
                cwd: SRC_DIR,
                strip: 1, // remove dist parent folder
            },
            ["dist"]
        );
    } catch (e) {
        console.log("Failed to clean and extract", e);
        return 1;
    }
}

async function main(): Promise<number | undefined> {

    if (config.use_github) {
      await buildFromGithubRepo(
        (config[TCHAP_ENV] as TchapConfigEnv)["tchap-web_github"].repo,
        (config[TCHAP_ENV] as TchapConfigEnv)["tchap-web_github"].branch
      );
    } else {
      const targetVersion: string | undefined =
        (config[TCHAP_ENV] as TchapConfigEnv)["tchap-web_version"];
      const filename: string | undefined =
        (config[TCHAP_ENV] as TchapConfigEnv)["tchap-web_archive_name"];
      await buildFromArchive(targetVersion, filename);
    }
    console.log("Done!");
    return 0;
}


main()
    .then((ret) => {
        process.exit(ret);
    })
    .catch((e) => {
        console.error(e);
        process.exit(1);
    });
