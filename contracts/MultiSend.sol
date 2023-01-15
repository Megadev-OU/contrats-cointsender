// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";

contract MultiSend {
    using SafeMath for uint256;
    using SafeERC20 for ERC20;
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

        uint256 currentSum = 0;
        uint256 taxes = 0;

        for (uint256 i = 0; i < recipients.length; i++) {
            require(amounts[i] > 0);
            currentSum = currentSum + amounts[i];
            uint256 FEE = amounts[i].mul(percent).div(10000);
            uint256 _amount = amounts[i].sub(FEE);
            require(currentSum <= msg.value);
            taxes = taxes + FEE;
            recipients[i].transfer(_amount);
        }
        payable(owner).transfer(taxes);
    }

    function multiSendDiffToken(
        address[] memory recipients,
        uint256[] memory amounts,
        address token
    ) public {
        require(recipients.length > 0);
        require(recipients.length == amounts.length);

        uint256 currentSum = 0;
        uint256 taxes = 0;
        for (uint256 i = 0; i < recipients.length; i++) {
            require(amounts[i] > 0);
            currentSum = currentSum + amounts[i];
            uint256 FEE = amounts[i].mul(percent).div(10000);
            uint256 _amount = amounts[i].sub(FEE);
            require(currentSum <= ERC20(token).balanceOf(msg.sender));
            taxes = taxes + FEE;
            ERC20(token).safeTransferFrom(msg.sender, recipients[i], _amount);
        }
         ERC20(token).safeTransferFrom(msg.sender, owner, taxes);
    }

}
