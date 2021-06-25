const fs = require('fs-extra')
const path = require('path')
const execa = require('execa')

async function getTargetTriple() {
  const rustTargetInfo = JSON.parse(
    (
      await execa(
        'rustc',
        ['-Z', 'unstable-options', '--print', 'target-spec-json'],
        {
          env: {
            RUSTC_BOOTSTRAP: 1
          }
        }
      )
    ).stdout
  )
  return rustTargetInfo['llvm-target']
}

async function main() {
  const resourceDirMap = {
    darwin: 'macos',
    win32: 'windows',
    linux: 'linux'
  }
  const resourcesPath = path.resolve(__dirname, `../resources/${resourceDirMap[process.platform]}`)
  const targetResourcesPath = path.resolve(__dirname, `../src-tauri/resources`)
  fs.copySync(resourcesPath, targetResourcesPath)
  if (!fs.existsSync(targetResourcesPath)) {
    fs.mkdirSync(targetResourcesPath)
  }

  const sidecarName = 'refine'
  const sidecarExtension = process.platform === 'win32' ? '.exe' : ''
  const targetTriple = await getTargetTriple()
  const sidecarBinPath = path.resolve(__dirname, '../src-tauri/binaries')
  if (!fs.existsSync(sidecarBinPath)) {
    fs.mkdirSync(sidecarBinPath)
  }
  fs.copyFileSync(
    path.resolve(__dirname, `../binaries/${sidecarName}${sidecarExtension}`),
    path.join(sidecarBinPath, `${sidecarName}-${targetTriple}${sidecarExtension}`)
  )
}

main()
