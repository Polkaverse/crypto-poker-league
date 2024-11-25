// async function timeout(flag, i) {


//     if (!flag) {
//         setTimeout(() => {
//             console.log("Inside setTimeout, i: ", i)
//             timeout(true, 1);
//         }, 2000);
//     }

//     return 1;
// }

function sleep(ms) {
    console.log("🚀 ~ file: asyncfordemo.mjs ~ line 15 ~ sleep ~ ms", ms);
    return new Promise((resolve) => {
        setTimeout(resolve, ms);
    });
}

function timeoutPromise(i) {
    console.log("val of i",i)
    sleep(5000)
    return i // return (5000)
}


// async function main() {
//     for (let i = 0; i < 3; i++) {
//         console.log("🚀 ~ file: asyncfordemo.mjs ~ line 11 ~ main ~ i", i);        

//         let res = await timeout(false,i);

//         if(res){
//             console.log("finish")

//         }
//     }
// }

// main()

async function main2() {
    for (let i = 0; i < 3; i++) {
        console.log("🚀 ~ file: asyncfordemo.mjs ~ line 11 ~ main ~ i", i);

        // async function timeout() {
        //     setTimeout(() => {
        //         console.log("Inside timeout>>setTimeout, i: ", i)
        //     }, 2000)
        // }
        // await timeout()

        await timeoutPromise(i);
        await timeoutPromise(i+1);

        console.log("finish")
    }
}

main2()

// function main3(i) {
//     console.log("🚀 ~ file: asyncfordemo.mjs ~ line 33 ~ main3 ~ i", i);
//     if(i<3){
//         setTimeout((i) => {
//             return main3(i + 1)
//         }, 2000)
//     }
// }

// main3(0);

// function main() {
//     for (let i = 0; i < 3; i++) {
//         console.log("🚀 ~ file: asyncfordemo.mjs ~ line 11 ~ main ~ i", i);



//         new Promise(f => setTimeout(f, 20000)).then(
//             sleep(5000),
//             console.log("Inside set timeout i:", i)
//         );

//         // new Promise((resolve, reject) => {
//         //     resolve(setTimeout(() => {
//         //         console.log("Inside setTimeout, i: ", i)
//         //     }, 2000))
//         // })


//         // .then(console.log("Inside .then i: ", i));


//         console.log("finish")

//     }
// }

// main()