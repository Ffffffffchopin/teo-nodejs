const { App, getModelClass } = require("./index.js")
const Decimal = require('decimal.js')
const User = getModelClass('User')

const app = new App()

app.transform("replaceComWithIo", async function (input) {
  return input.replace(/com$/, 'io')
})

app.transform("10times", async function (input) {
  return Decimal((input || Decimal(5)) * 10)
})

app.validate("oddLength", (s) => s.length % 2 == 0)

app.callback("print", async (v) => {
  let user = await User.create({ email: `${v}@gmail.com` });
  await user.save()
  console.log(user)
})

app.run()
