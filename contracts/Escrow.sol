// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract MilestoneEscrow {
    address public payer;
    address public payee;
    uint256 public amount;
    bool public released;

    constructor(address _payee) payable {
        payer = msg.sender;
        payee = _payee;
        amount = msg.value;
        released = false;
    }

    function release() public {
        require(!released, "already released");
        require(msg.sender == payer, "only payer can release");
        released = true;
        payable(payee).transfer(amount);
    }

    function refund() public {
        require(!released, "already released");
        require(msg.sender == payee, "only payee can request refund");
        released = true;
        payable(payer).transfer(amount);
    }
}
