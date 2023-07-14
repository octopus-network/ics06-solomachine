# ICS06 Solo Machine Client by rust

Current this implement just impl Single Signatue, currently only supports single sign-on.

implement [ics06-solo-machine-client](https://github.com/cosmos/ibc/blob/main/spec/client/ics-006-solo-machine-client/README.md) by rust

v2: reference to ibc-go v5.3.0 [ics06-solo-machine-client](https://github.com/cosmos/ibc-go/tree/v5.3.0/modules/light-clients/06-solomachine)

v3: reference to ibc-go main [ics06-solo-machine-client](https://github.com/cosmos/ibc-go/tree/main/modules/light-clients/06-solomachine)

## todo

[ ] Multi Signature

## issue

- ics06 solomachine client consensus state don't have CommitmentRoot field, current just add for ics06 solomachine client consensus state a temp root(CommitmentRoot) field and give value is publicKey Bytes.
