#!/usr/bin/env zx
const fs = require('fs')
const gncBinary = '../target/release/GNC'

let totalCount = 0
let passCount = 0
let failList = []

// Pass cases
const passCasePath = './minidecaf-tests/testcases/step'
for (let i = 1; i <= 12; i++) {
    const dir = await fs.promises.opendir(`${passCasePath}${i}`)
    for await (const dirent of dir) {
        const [fileName, fileType] = dirent.name.split('.')
        if (fileType === 'c') {
            console.log(`${passCasePath}${i}/${fileName}.c`)
            totalCount += 1

            // gnc result
            let gncResult
            try {
                await $`${gncBinary} ${passCasePath}${i}/${fileName}.c`
                await $`clang ${passCasePath}${i}/${fileName}.asm`
                gncResult = await $`./a.out`
            } catch (p) {
                gncResult = p
            }

            // clang result
            let clangResult
            try {
                await $`clang ${passCasePath}${i}/${fileName}.c`
                clangResult = await $`./a.out`
            } catch (p) {
                clangResult = p
            }

            console.log(gncResult)
            console.log(clangResult)
            if (gncResult.exitCode === clangResult.exitCode) {
                console.log('[PASS]')
                passCount += 1
            } else {
                failList.push(`${passCasePath}${i}/${fileName}.c`)
            }
        }
    }
}

// Fail cases
const failCasePath = './minidecaf-tests/failcases/step'
for (let i = 1; i <= 12; i++) {
    let dir = null
    try {
        dir = await fs.promises.opendir(`${failCasePath}${i}`)
    } catch (e) {
    }
    if (dir) {
        for await (const dirent of dir) {
            const [fileName, fileType] = dirent.name.split('.')
            if (fileType === 'c') {
                console.log(`${failCasePath}${i}/${fileName}.c`)
                totalCount += 1

                // gnc result
                let gncResult
                try {
                    await $`${gncBinary} ${failCasePath}${i}/${fileName}.c`
                    await $`clang ${failCasePath}${i}/${fileName}.asm`
                    gncResult = await $`./a.out`
                } catch (p) {
                    gncResult = p
                }

                console.log(gncResult)
                if (gncResult.exitCode !== 0) { // should have error
                    console.log('[PASS]')
                    passCount += 1
                } else {
                    failList.push(`${failCasePath}${i}/${fileName}.c`)
                }
            }
        }
    }
}

console.log('\nTest Done:')
console.log(`Passed ${passCount} / ${totalCount}`)
console.error(`Failed ${totalCount - passCount} / ${totalCount}`)
console.warn(`Failed cases are ${JSON.stringify(failList, null, 2)}`)