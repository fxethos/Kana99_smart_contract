/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  checkProgram,
  sendPayouts,
  //reportGreetings,
  //reportGreetings,
} from './payout';

async function main() {
  console.log("connect to solana account...");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();

  // Check if the program has been deployed
  await checkProgram();

  // Say hello to an account
  await sendPayouts();

  // Find out how many times that account has been greeted
  //await reportGreetings();

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
