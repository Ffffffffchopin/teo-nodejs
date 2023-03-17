const { App } = require("./index.js")
const Decimal = require('decimal.js')

async function main() {
  const app = new App()
  app.transform("replaceComWithIo", async function (input) {
    return input.replace(/com$/, 'io')
  })
  app.transform("10times", async function (input) {
    return Decimal((input || Decimal(5)) * 10)
  })
  await app.run()
}

main()
