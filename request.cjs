const request = require('request');

async function main() {
  var options = {
    method: 'POST',
    url: 'https://aip.baidubce.com/oauth/2.0/token?client_id=Wq2mGzgQX9Q13f2tg1gZtQyy&client_secret=zuxcm9DtemprEg8rf2FzTgLyNG2KZSDf&grant_type=client_credentials',
    headers: {
      'Content-Type': 'application/json',
      Accept: 'application/json',
    },
  };

  request(options, function (error, response) {
    if (error) throw new Error(error);
    console.log(response.headers);
  });
}

main();
