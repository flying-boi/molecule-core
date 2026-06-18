// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleDAO {
    struct Proposal {
        string description;
        uint256 votesFor;
        uint256 votesAgainst;
        uint256 deadline;
        bool executed;
    }

    Proposal[] public proposals;
    mapping(address => uint256) public votingPower;

    function newProposal(string memory description, uint256 votingPeriod) public {
        proposals.push(Proposal({
            description: description,
            votesFor: 0,
            votesAgainst: 0,
            deadline: block.timestamp + votingPeriod,
            executed: false
        }));
    }

    function vote(uint256 proposalId, bool support) public {
        require(block.timestamp < proposals[proposalId].deadline, "voting closed");
        uint256 power = votingPower[msg.sender];
        require(power > 0, "no voting power");
        if (support) {
            proposals[proposalId].votesFor += power;
        } else {
            proposals[proposalId].votesAgainst += power;
        }
    }

    function setVotingPower(address voter, uint256 power) public {
        votingPower[voter] = power;
    }
}
