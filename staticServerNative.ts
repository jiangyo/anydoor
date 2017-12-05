
// import * as http from "http";
import * as express from 'express';
const app = express();
import conf from "./config/defaultConfig";
import chalk from "chalk";
import * as path from "path";
import * as fs from "fs";
import {promisify} from "util";

const stat = promisify(fs.stat);
const readdir = promisify(fs.readdir);



// const server = http.createServer((req, res) => {
// res.statusCode = 200;
// res.setHeader('Content-Type','text/html');
//     res.write('<html>');
//     res.write('<body>');
//     res.write('<p>');
//     res.write('Hello Http NodeJs !');
//     res.write('</p>');
//     res.write('</body>');
//     res.end('</html>');
// });


app.all('*',(req, res) => {

    const url  = req.url;
    const filePath = path.join(conf.root,url);
    // 判断是否文件夹
    fs.stat(filePath,(err, stats) => {
        if(err) {
            res.statusCode = 404;
            res.setHeader('Content-Type','text/plain');
            res.write(filePath);
            res.end(`${filePath} is not a directory or file`);
            return;
        }

        if(stats.isFile()){
            res.statusCode = 200;
            res.setHeader('Content-Type','text/html;charset=utf-8');
            const cws = fs.createReadStream(filePath,'utf-8').pipe(res);
            res.write(`${filePath} is a file`);

        }else if(stats.isDirectory()){
            fs.readdir(filePath,(err,files) => {
                res.statusCode = 200;
                res.setHeader('Content-Type','text/plain');
                res.write(`${filePath} is a director `);
                res.end(files.join(','));
            })
        }
    })

    

    try {

    }


});

const tserver = app.listen(conf.port,conf.hostname, ()=>{
    const addr =`${conf.hostname}:${conf.port}`;
    console.info(`server started at ${chalk.green(addr)}`);
});

