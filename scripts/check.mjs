import path from "path";

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


async function getLatestAlphaVersion() {

}


async function getLatestReleaseVersion() {
    
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