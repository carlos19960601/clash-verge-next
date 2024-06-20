import { execSync } from "child_process";
import fs from "fs-extra";
import { HttpsProxyAgent } from "https-proxy-agent";
import fetch from "node-fetch";
import path from "path";
import zlib from "zlib";

const cwd = process.cwd();
console.log(cwd)

const TEMP_DIR = path.join(cwd, "node_modules/.verge")
const FORCE = process.argv.includes("--force")

console.log(process.argv)



const arg1 = process.argv.slice(2)[0];
const arg2 = process.argv.slice(2)[1];

const target = arg1 === "--force" ? arg2 : arg1;


const PLATFORM_MAP = {

}

const {platform, arch} = target ? {platform: PLATFORM_MAP[target], arch: ARCH_MAP[target]} : process
console.log(platform, arch)

const SIDECAR_HOST = target ? target : execSync("rustc -vV").toString().match(/(?<=host: ).+(?=\s*)/g)[0]
console.log("SIDECAR_HOST: ", SIDECAR_HOST)


const META_MAP = {
    "darwin-x64": "mihomo-darwin-amd64-compatible"
}

const META_ALPHA_MAP = {
    "darwin-x64": "mihomo-darwin-amd64-compatible",
}

if (!META_MAP[`${platform}-${arch}`]) {
    throw new Error(`clash meta unsupported platform "${platform}-${arch}"`)
}

if (!META_ALPHA_MAP[`${platform}-${arch}`]) {
    throw new Error(`clash meta alpha unsupported platform "${platform}-${arch}"`)
}




let META_ALPHA_VERSION;
const META_ALPHA_VERSION_URL = "https://github.com/MetaCubeX/mihomo/releases/download/Prerelease-Alpha/version.txt";


async function downloadFile(url, path) {
    const options = {}

    const httpProxy = process.env.http_proxy || process.env.HTTP_PROXY || process.env.https_proxy || process.env.HTTPS_PROXY
    if (httpProxy) {
        options.agent = HttpsProxyAgent(httpProxy)
    }

    const response = await fetch(url, {...options, method: "GET", headers: {"Content-Type": "application/octet-stream"}})

    const buffer = await response.arrayBuffer()
    await fs.writeFile(path, new Uint8Array(buffer))

    console.log(`[INFO]: download finished "${url}"`);
}

async function getLatestAlphaVersion() {
    const options = {}

    const httpProxy = process.env.http_proxy || process.env.HTTP_PROXY || process.env.https_proxy || process.env.HTTPS_PROXY
    if (httpProxy) {
        options.agent = HttpsProxyAgent(httpProxy)
    }

    try {
        const response = await fetch(META_ALPHA_VERSION_URL, {
            ...options,
            method: "GET",
        })
        let v = await response.text()
        META_ALPHA_VERSION = v
        console.log(`Latest alpha version: ${META_ALPHA_VERSION}`);
    } catch(error) {
        console.error("Error fetching latest alpha version:", error.message);
        process.exit(1);
    } 
}

let META_VERSION;
const META_VERSION_URL =
  "https://github.com/MetaCubeX/mihomo/releases/latest/download/version.txt";

async function getLatestReleaseVersion() {
    const options = {}

    const httpProxy = process.env.http_proxy || process.env.HTTP_PROXY || process.env.https_proxy || process.env.HTTPS_PROXY
    if (httpProxy) {
        options.agent = HttpsProxyAgent(httpProxy)
    }

    try {
        const response = await fetch(META_VERSION_URL, {
            ...options,
            method: "GET",
        })
        let v = await response.text()
        META_VERSION = v.trim()
        console.log(`Latest release version: ${META_VERSION}`);
    } catch(error) {
        console.error("Error fetching latest release version:", error.message);
        process.exit(1);
    } 
}

const META_ALPHA_URL_PREFIX = `https://github.com/MetaCubeX/mihomo/releases/download/Prerelease-Alpha`;
function clashMetaAlpha() {
    const name = META_ALPHA_MAP[`${platform}-${arch}`];
    const isWin = platform === "win32"
    const urlExt = isWin ? "zip" : "gz"
    const downloadURL = `${META_ALPHA_URL_PREFIX}/${name}-${META_ALPHA_VERSION}.${urlExt}`
    const exeFile = `${name}${isWin ? ".exe" : ""}`;
    const zipFile = `${name}-${META_ALPHA_VERSION}.${urlExt}`;


    return {
        name: "verge-mihomo-alpha",
        targetFile: `verge-mihomo-alpha-${SIDECAR_HOST}${isWin ? ".exe" : ""}`,
        exeFile,
        zipFile,
        downloadURL,
    }
}

const META_URL_PREFIX = `https://github.com/MetaCubeX/mihomo/releases/download`;
function clashMeta() {
    const name = META_MAP[`${platform}-${arch}`];
    const isWin = platform === "win32";
    const urlExt = isWin ? "zip" : "gz";
    const downloadURL = `${META_URL_PREFIX}/${META_VERSION}/${name}-${META_VERSION}.${urlExt}`;
    const exeFile = `${name}${isWin ? ".exe" : ""}`;
    const zipFile = `${name}-${META_VERSION}.${urlExt}`;
  
    return {
      name: "verge-mihomo",
      targetFile: `verge-mihomo-${SIDECAR_HOST}${isWin ? ".exe" : ""}`,
      exeFile,
      zipFile,
      downloadURL,
    };
}



async function resolveSidecar(binInfo) {
    const { name, targetFile, zipFile, exeFile, downloadURL } = binInfo;

    const sidecarDir = path.join(cwd, "src-tauri", "sidecar");
    const sidecarPath = path.join(sidecarDir, targetFile);

    await fs.mkdirp(sidecarDir);
    if (!FORCE && (await fs.pathExists(sidecarPath))) return;

    const tempDir = path.join(TEMP_DIR, name)
    const tempZip = path.join(tempDir, zipFile)
    const tempExe = path.join(tempDir, exeFile)

    await fs.mkdirp(tempDir)
        
    try {
        if (!(await fs.pathExists(tempZip))) {
            await downloadFile(downloadURL, tempZip)
        }

        if (zipFile.endsWith(".zip")) {

        } else if (zipFile.endsWith(".tgz")) {

        } else {
            // gz
            const readStream = fs.createReadStream(tempZip);
            const writeStream = fs.createWriteStream(sidecarPath);
            await new Promise((resolve, reject) => {
                const onError = (error) => {
                    console.error(`[ERROR]: "${name}" gz failed:`, error.message);
                    reject(error);
                }
                readStream
                    .pipe(zlib.createGunzip().on("error", onError))
                    .pipe(writeStream)
                    .on("finish", ()=>{
                        console.log(`[INFO]: "${name}" gunzip finished`);
                        execSync(`chmod 755 ${sidecarPath}`);
                        console.log(`[INFO]: "${name}" chmod binary finished`);
                        resolve();
                    })
                    .on("error", onError)
                
            })
        }
    } catch (error) {
        await fs.remove(sidecarPath)
        throw error
    } finally {
        await fs.remove(tempDir)
    }
}

const tasks = [
    {
        name: "verge-mihomo-alpha",
        func: ()=> getLatestAlphaVersion().then(()=>resolveSidecar(clashMetaAlpha())),
        retry: 5
    },
    {
        name: "verge-mihomo",
        func: ()=>getLatestReleaseVersion().then(()=>resolveSidecar(clashMeta())),
        retry: 5
    }
]



async function runTask() {
    const task = tasks.shift()
    if (!task) return

    for (let i = 0; i < task.retry; i++) {
        try {
            await task.func();
            break
        }catch(err) {
            console.error(`[ERROR]: task::${task.name} try ${i} ==`, err.message);
            if (i === task.retry - 1) throw err
        }
    }

    return runTask()
}

runTask()