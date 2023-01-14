// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract MultiSend is Ownable{

    function multiSendDiffEth(
        address payable[] memory recipients,
        uint256[] memory amounts,
        uint256[] memory fee
    ) public payable {
        require(recipients.length > 0);
        require(recipients.length == amounts.length);
        uint256[] memory newSumAfterFee;

        uint256 currentSum = 0;
        uint256 taxes = 0;

        for (uint256 i = 0; i < recipients.length; i++) {
            require(amounts[i] > 0);
            currentSum = currentSum + amounts[i];
            require(currentSum <= msg.value);
        }

        for (uint256 i = 0; i < recipients.length; i++) {
            newSumAfterFee[i] = amounts[i] - fee[i];
            taxes += fee[i];
            recipients[i].transfer(newSumAfterFee[i]);
        }
        payable(Ownable.owner()).transfer(taxes);
    }

    function multiSendDiffToken(
        address payable[] memory recipients,
        uint256[] memory amounts,
        uint256[] memory fee,
        address token
    ) public {
        uint256[] memory newSumAfterFee;
        require(recipients.length > 0);
        require(recipients.length == amounts.length);

        uint256 currentSum = 0;
        uint256 taxes = 0;

        for (uint256 i = 0; i < recipients.length; i++) {
            require(amounts[i] > 0);
            currentSum = currentSum + amounts[i];
            require(currentSum <= ERC20(token).balanceOf(msg.sender));
        }

        for (uint256 i = 0; i < recipients.length; i++) {
            newSumAfterFee[i] = amounts[i] - fee[i];
            taxes += fee[i];
            ERC20(token).transferFrom(msg.sender, recipients[i], newSumAfterFee[i]);
        }
        ERC20(token).transferFrom(msg.sender, Ownable.owner(), taxes);
    }

}
