// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract MultiSend {
    address public owner;
    uint256 public percent;
    // Defining a constructor
    constructor(address _owner) {
        owner = _owner;
        percent = 10; // 0.1%
    }

    modifier isOwner() {
        require(msg.sender == owner, "Caller is not the owner");
        _;
    }

    function changePercentage(uint256 _percent) public isOwner {
        percent = _percent;
    }

    function multiSendDiffEth(
        address payable[] memory recipients,
        uint256[] memory amounts
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
            newSumAfterFee[i] = amounts[i] - (amounts[i] * percent / 10000);  // 100 - (100 * 10 / 10000) = 99.9
            taxes += (amounts[i] / 10000 * percent);  // (100 * 10 / 10000) = 0.1
            recipients[i].transfer(newSumAfterFee[i]);
        }
        payable(owner).transfer(taxes);
    }

    function multiSendDiffToken(
        address payable[] memory recipients,
        uint256[] memory amounts,
        address token
    ) public {
        require(recipients.length > 0);
        require(recipients.length == amounts.length);
        uint256[] memory newSumAfterFee;

        uint256 currentSum = 0;
        uint256 taxes = 0;

        for (uint256 i = 0; i < recipients.length; i++) {
            require(amounts[i] > 0);
            currentSum = currentSum + amounts[i];
            require(currentSum <= IERC20(token).balanceOf(msg.sender));
        }

        for (uint256 i = 0; i < recipients.length; i++) {
            newSumAfterFee[i] = amounts[i] - (amounts[i] * percent / 10000);  // 100 - (100 * 10 / 10000) = 99.9
            taxes += (amounts[i] / 10000 * percent);  // (100 * 10 / 10000) = 0.1
            SafeERC20.safeTransferFrom(IERC20(token), msg.sender, recipients[i], newSumAfterFee[i]);
        }
        SafeERC20.safeTransferFrom(IERC20(token), msg.sender, owner, taxes);
    }

}
