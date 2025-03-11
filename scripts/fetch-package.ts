#!/usr/bin/env -S npx tsx --resolveJsonModule

import * as path from "node:path";
import { createWriteStream, promises as fs } from "node:fs";
import { promises as stream } from "node:stream";
import * as tar from "tar";

import { tchapConfig } from "../package.json";
import { exec as execCallback } from "node:child_process";
import { promisify } from "node:util";

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
    } catch (e) {
        // Directory might not exist, that's fine
    }
    
    // Clone the specific branch
    await exec(`git clone --depth 1 --branch ${branch} ${repoUrl} ${targetDir}`);
    console.log(`Repository cloned successfully to ${targetDir}`);
}

async function buildFromLocalRepo(targetDir: string) {
    console.log(`Building from local repo ${targetDir}`);
    console.log(`Install dependencies`);
    await exec(`yarn install`, { cwd: targetDir });
    console.log(`Building tchap web`);
    await exec(`yarn build`, { cwd: targetDir });
    console.log(`Build completed successfully`);

    console.log(`Copying webapp dist folder to src folder`);
    // copy the dist folder to the src folder
    await fs.cp(path.join(targetDir, "webapp/"), SRC_DIR, { recursive: true });


}


async function buildFromGithubRepo(repoUrl: string, branch: string) {
    try {
        console.log("Building from github repo", branch);
        const targetDir = ARCHIVE_DIR + "/" + branch;
        await cloneGitHubRepo(repoUrl, branch, targetDir);
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

    if (tchapConfig!["tchap-web_github"]?.use_github) {
      await buildFromGithubRepo(tchapConfig!["tchap-web_github"]?.repo, tchapConfig!["tchap-web_github"]?.branch);
    } else {
      const targetVersion: string | undefined =
        tchapConfig!["tchap-web_version"];
      const filename: string | undefined =
        tchapConfig!["tchap-web_archive_name"];
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
