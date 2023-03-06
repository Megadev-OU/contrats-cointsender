// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract ERC20Mock is ERC20 {
    constructor(
        string memory name,
        string memory symbol,
        uint256 supply
    ) ERC20(name, symbol) {
        _mint(msg.sender, supply);
    }

    function mint(uint256 amount) public {
        _mint(_msgSender(), amount);
    }

    function mint(address receiver, uint256 amount) public {
        _mint(receiver, amount);
    }

}