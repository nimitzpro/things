document.addEventListener('DOMContentLoaded', init);

let chars = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];

let letters = [];
let letters2 = [];
let canvas;
let ctx;   

let interval;
let playing = true;

let lastScrollTop;


function init(){
    lastScrollTop = pageYOffset || document.documentElement.scrollTop;

    canvas = document.querySelector('canvas');
    ctx    = canvas.getContext('2d');

    // for(let i of chars){
    //     let x = i.toUpperCase();
    //     chars.push(x);
    // }

    console.log(chars)

    canvas.height = window.innerHeight;
    canvas.width = window.innerWidth;

    ctx.fillStyle = "black";
    ctx.fillRect(0,0, canvas.width, canvas.height);

    interval = window.setInterval(loop, 33);

    // Text things

    typing();

    // ----------

    document.addEventListener('keydown', (event) => handleKey(event));

    document.addEventListener('scroll', handleScroll);

    for(let i = 0; i < 300; i++){
        let char = Math.random() > 0.2 ? Math.random() > 0.4 ? Math.floor(33 + Math.random()*94) : Math.floor(921 + Math.random()*48) : Math.floor(12353 + Math.random()*179);
        // if(char.length < 4){
            // char += "0";
        // }


        // Math.random()*canvas.width [x]
        // Math.random()*canvas.height [y]


        let letter = {char: char, x: Math.random()*canvas.width, y: 0-Math.random()*canvas.height, timeSinceChange: Math.floor(Math.random()*100)};
        // let letter = {char: char, x: 100 + 35*letters.length, y: 100, timeSinceChange: Math.floor(Math.random()*100)};
        // "\\u" + 
        letters.push(letter);
    }
    for(let i = 0; i < 300; i++){
        let char = Math.random() > 0.2 ? Math.random() > 0.4 ? Math.floor(33 + Math.random()*94) : Math.floor(921 + Math.random()*48) : Math.floor(12353 + Math.random()*179);
        // if(char.length < 4){
            // char += "0";
        // }


        // Math.random()*canvas.width [x]
        // Math.random()*canvas.height [y]


        let letter = {char: char, x: Math.random()*canvas.width, y: 0-Math.random()*canvas.height, timeSinceChange: Math.floor(Math.random()*100)};
        // let letter = {char: char, x: 100 + 35*letters.length, y: 100, timeSinceChange: Math.floor(Math.random()*100)};
        // "\\u" + 
        letters2.push(letter);
    }
    console.log(letters)
}

function loop(){
    canvas.height = window.innerHeight;
    canvas.width = window.innerWidth;
    
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    for(let i = 0; i < letters.length; i++){
        ctx.fillStyle = "green";
        ctx.font = "20px courier";
        ctx.fillText(String.fromCharCode(letters[i].char), letters[i].x, letters[i].y);
        // ctx.fillText(i+1, letters[i].x, letters[i].y + 30 + (i % 2 === 0 ? 10 : 0));
        
        if(letters[i].y > canvas.height*1.5){
            letters[i].y = 0 - Math.random() * canvas.height;
            letters[i].x = 0 + Math.random() * canvas.width;
        }
        letters[i].y++;
        
        if(letters[i].timeSinceChange <= 0){
            // letters[i].char = Math.floor(33 + Math.random()*94);
            letters[i].char = Math.random() > 0.2 ? Math.random() > 0.4 ? Math.floor(33 + Math.random()*94) : Math.floor(921 + Math.random()*48) : Math.floor(12353 + Math.random()*179);
            letters[i].timeSinceChange = Math.floor(Math.random()*100);
        }
        letters[i].timeSinceChange--;
    }

    for(let i = 0; i < letters2.length; i++){
        ctx.fillStyle = "rgba(153,255,153,0.5)";
        ctx.font = "10px courier";
        ctx.fillText(String.fromCharCode(letters2[i].char), letters2[i].x, letters2[i].y);
        // ctx.fillText(i+1, letters[i].x, letters[i].y + 30 + (i % 2 === 0 ? 10 : 0));
        
        if(letters2[i].y > canvas.height*1.5){
            letters2[i].y = 0 - Math.random() * canvas.height;
            letters2[i].x = 0 + Math.random() * canvas.width;
        }
        letters2[i].y += 0.5;
        
        if(letters2[i].timeSinceChange <= 0){
            // letters[i].char = Math.floor(33 + Math.random()*94);
            letters2[i].char = Math.random() > 0.2 ? Math.random() > 0.4 ? Math.floor(33 + Math.random()*94) : Math.floor(921 + Math.random()*48) : Math.floor(12353 + Math.random()*179);
            letters2[i].timeSinceChange = Math.floor(Math.random()*100);
        }
        letters2[i].timeSinceChange--;
    }
}

function handleKey(e){
    console.log("clicked", e.keyCode)
    if(e.keyCode === 83){
        if(playing){
            window.clearInterval(interval, 33);
            console.log(letters);
            playing = false;
        }
        else{
            interval = window.setInterval(loop, 33);
            playing = true;
        }
    }
}

function handleScroll(){
    let st = pageYOffset || document.documentElement.scrollTop;
    if(st > lastScrollTop){
        for(let i of letters){
            i.y -= 5;
        }
        for(let j of letters2){
            j.y -= 2.5;
        }
    }
    else{
        for(let i of letters){
            i.y += 5;
        }
        for(let j of letters2){
            j.y += 2.5;
        } 
    }
    lastScrollTop = st <= 0 ? 0 : st;
}

let h1 = document.querySelector('h1');
let cursor = false;

function textEffect(){
    if(cursor){
        h1.innerHTML = h1.innerHTML.slice(0, h1.innerHTML.length - 1) + String.fromCharCode(8199);
        // h1.innerHTML = h1.innerHTML.slice(0, h1.innerHTML.length - 1) + String.fromCharCode(8198);
        // h1.innerHTML = h1.innerHTML.slice(0, h1.innerHTML.length - 1) + " ";
        cursor = false;
    }
    else{
        // h1.innerHTML = h1.innerHTML.slice(0, h1.innerHTML.length - 1) + String.fromCharCode(9615);
        h1.innerHTML = h1.innerHTML.slice(0, h1.innerHTML.length - 1) + String.fromCharCode(9144);
        cursor = true;
    }
}

let heading = "Testing";
let i = 0;

function typing(){
    if(i < heading.length){
        h1.innerHTML += heading.charAt(i);
        i++;
        setTimeout(typing, 100);
    }
    else{
        h1.innerHTML += "&#8199;";
        window.setInterval(textEffect, 750);
    }
}