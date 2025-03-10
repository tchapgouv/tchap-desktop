#!/usr/bin/env -S npx tsx --resolveJsonModule

import * as path from "node:path";
import { createWriteStream, promises as fs } from "node:fs";
import { promises as stream } from "node:stream";
import * as tar from "tar";

import { tchapConfig } from "../package.json";
import { exec } from "node:child_process";

const PACKAGE_URL_PREFIX = "https://github.com/tchapgouv/tchap-web-v4/releases/download/"

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


async buildFromGithubRepo() {

}

async function main(): Promise<number | undefined> {
    let srcDir = "src";
    let targetVersion: string | undefined = tchapConfig!["tchap-web_version"];
    let filename: string | undefined = tchapConfig!["tchap-web_archive_name"];
    let url: string | undefined = PACKAGE_URL_PREFIX + `tchap-${targetVersion}` + "/" + filename;
    let archiveDir = "archives";
    let haveArchive = false;
    
    const selectedArchivePath = path.join(archiveDir, filename);
    
    // check if we already downloaded the archive yet
    try {
        await fs.stat(selectedArchivePath);
        console.log(selectedArchivePath + " already exists");
        // means we don't need to download it again
        haveArchive = true;
    } catch (e: any) {
        console.log('the archive does not exist, proceed to download it')
    }
    
    try {
        if (!haveArchive) {
            console.log('downling archive ');
            await downloadToFile(url, selectedArchivePath);
        }
    } catch (e) {
        console.log("Failed to download " + url, e);
        return 1;
    }


    try {
        // we extract the downloaded file to the src folder
        // tar will overwrite the existing files and folder
        console.log(`Extracting the archives to ${srcDir}`);
        await tar.x({
            file: selectedArchivePath,
            cwd: srcDir,
            strip: 1 // remove dist parent folder
        }, ['dist']);
    } catch (e) {
        console.log("Failed to clean and extract", e);
        return 1;
    }

    console.log("Done!");
}

main()
    .then((ret) => {
        process.exit(ret);
    })
    .catch((e) => {
        console.error(e);
        process.exit(1);
    });
