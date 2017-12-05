import * as express from 'express';
const app = express();

app.get('/',(req,res) => res.send('woshishouye'));
app.get('/test',(req,res) => res.send('测试页面'));

const server = app.listen(8080,'localhost',() => {
    console.log("服务器启动");
})

