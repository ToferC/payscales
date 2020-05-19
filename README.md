# payscales
API for Government of Canada pay/salary scales for classifications and levels. The intent is to have a central, easy to access and use resource for accurate pay information. This will reduce duplication and potential errors across organizations and serve as an example of how APIs can support day-to-day business in government.

This is also a learning project in Rust and my first foray into GraphQL development.

You can see the running work in progress here: https://gc-payscales.herokuapp.com/graphiql

A sample query looks like this:

```graphql
{
  group(identifier: FS) {
    payscales {
      steps
      level
      currentRatesOfPay{
        salary(step: 1)
      }
    }
    payForLevelAndStepToday(level: 2, step: 1)
  }
}
```
