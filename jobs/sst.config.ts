/// <reference path="./.sst/platform/config.d.ts" />

export default $config({
  app(input) {
    return {
      name: "whatsexpense-jobs",
      removal: input?.stage === "production" ? "retain" : "remove",
      home: "aws",
      providers: {
        aws: {
          region: "ap-southeast-1",
          profile: "linrium",
        },
      },
    };
  },
  async run() {
    new sst.aws.Cron("UpdateLatestExchangeRate", {
      job: {
        handler: "src/cron/exchange-rate/index.handler",
        environment: {
            BASE_URL: getExchangeRateApiUrl(),
        }
      },
      schedule: "rate(1 day)",
    });
  },
});

const getExchangeRateApiUrl = () => {
  if ($app.stage === "prod") {
    return ""
  }

  return ""
}