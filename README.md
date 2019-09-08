# Ergo Crowdfunding CLI Tool

The Ergo Crowdfunding CLI tool allows you to participate and interact with Crowdfunding Campaigns on Ergo easily.


```
Usage: 
        ergo_cf back
        ergo_cf create <campaign-deadline> <campaign-goal> 
        ergo_cf delete
        ergo_cf info
        ergo_cf import <file-path>
        ergo_cf export
        ergo_cf track <campaign-name> <campaign-address> <campaign-deadline> <campaign-goal>
```

## How It Works

In order to interact with a campaign you first need to track it inside of the CLI tool. You have three ways of doing so:
 - You `create` a new campaign locally (which you can then share with others)
 - You `import` an already created campaign via file which somebody else already created/exported
 - You `track` an already created campaign by manually inputting the relevant data.

Once a campaign is tracked via one of three above methods you can then interact with the campaign:
 - Using `back` to send Erg to the P2S address generated for locking your funds under the Crowdfund Script
 - Using `info` to display information about a tracked campaign (including previous backing txs if you backed it before)
 - Using `export` to export a tracked campaign into a file in the `export` folder which you can then share with others.
 - Using `delete` to delete a tracked campaign from local storage.

## Example - How To Use

Let's use the very first crowdfund campaign ever conducted on Ergo as an example. We will interact with the campaign by tracking it first:
```
./ergo_cf track "First Ergo Crowdfund" 9gBSqNT9LH9WjvWbyqEvFirMbYp4nfGHnoWdceKGu45AKiya3Fq 50000 500
```
```
Ergo Crowdfund CLI
------------------
Valid Campaign information submitted. This campaign is now being tracked:

Campaign Name: First Ergo Crowdfund
Campaign Address: 9gBSqNT9LH9WjvWbyqEvFirMbYp4nfGHnoWdceKGu45AKiya3Fq
Campaign Deadline Block: 50000
Campaign Goal: 500

```
At any point if you wish to see the information above again about the campaign you can always use `info`.

We will now proceed to back the Crowdfund Campaign and send 1 Erg.
```
./ergo_cf back
```

You will be asked which tracked campaign you wish to back:

```
Ergo Crowdfund CLI
------------------
1. First Ergo Crowdfund

Which campaign would you like to select?
```

Then afterwards how many Erg you wish to send. 

Once that is all submitted then the Crowdfund tool will then do the rest to back the Campaign. It fills in the Crowdfund script for you, generates the P2S Address via a POST to your running & unlocked node/wallet, and then submits the transaction to the P2S Address thusly participating and backing the campaign. If successful you will see something similar to the information below:

```
Ergo Crowdfund CLI
------------------
Campaign Name: First Ergo Crowdfund
Campaign Address: 9gBSqNT9LH9WjvWbyqEvFirMbYp4nfGHnoWdceKGu45AKiya3Fq
Campaign Deadline Block: 50000
Campaign Goal: 500
Address You Used To Back: 9gBSqMT9LH9WjvWbyqEvFirMbYp4nfGHnoWdceKGu45APiya3Gl
P2S Address Paid To: nA46m9Zz6DsA4yAPj7E9MVDU5BcdZFvGJq6RooKfk8yisehVHtX2QtjtjXCzsJmQZDTJRZ8DtscG7T8tm67Zhf94atLDoeBXKFUEYDce3gxKgu8Fpn9ZbpoqdcqWFfS
Backing Txs:
   - 12bb9599c0cb47436je97b3506c9dd6be0a46421cd793d1245491004504c9817: 1 Erg
```

This information is then saved locally and is available at any time by calling `info` and selecting the campaign.


## Setup

 1. Install latest Rust stable by going to: https://www.rust-lang.org/learn/get-started
 2. Clone the repository via 
 ```
 git clone https://github.com/robkorn/ergo-crowdfunding-cli
 ```
 3. Run the `build_release.sh` script to compile the tool:
 ```
 sh build_release.sh
 ```
 4. Enter the newly created `ergo_cf_release` folder and use the CLI tool from here. (You are free to move the folder after compiling, but you need to keep the binary in the folder because the Crowdfund tool saves some local state there too.)
 ```
 cd ergo_cf_release
 ./ergo_cf create 50000 500
 ```
 5. Upon first running you will be asked to enter your api key for sending requests to your node. Fill it out accordingly.
 6. Lastly, the Crowdfund tool assumes you have your node running locally via API port `9052`. If that is not the case then edit the `node.ip` file with the correct ip/port to access your node. (And make sure your wallet is unlocked as well.)
 7. Enjoy creating and participating in Crowdfunding campaigns on Ergo.



### Notes

 - Currently a backer may only send whole integer value amounts of Erg to a campaign.
 - Currently there is no interface for a creator of a Campaign to check if it has succeeded nor to collected all of the funds upon success

More features/updates are to come, especially when [EIP-1](https://github.com/ergoplatform/eips/blob/master/eip-0001.md) is finished.

### Further Info

For more information on how crowdfunding works, read page 6 of the [ErgoScript Whitepaper](https://docs.ergoplatform.com/ErgoScript.pdf).
