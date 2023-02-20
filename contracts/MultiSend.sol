// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

// import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/math/SafeMathUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";


contract MultiSend is  UUPSUpgradeable, OwnableUpgradeable {
    using SafeMathUpgradeable for uint256;
    using SafeERC20Upgradeable for IERC20Upgradeable;

    uint256 public percent;
    address public bank;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function _authorizeUpgrade(address newImplementation)
        internal
        virtual
        override
        onlyOwner
    {}

    function initialize() public initializer {
        __Ownable_init();
        __UUPSUpgradeable_init();
        
        percent = 10;
        // 0.1%
        bank = 0x3Ff0Dc6514d719152692188bD6F0771ADe370852;
    }

    function changePercentage(uint256 _percent) public onlyOwner {
        percent = _percent;
    }

    function changeBankAddress(address _bank) public onlyOwner {
        bank = _bank;
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
            uint256 FEE = amounts[i].mul(percent).div(10000);
            currentSum = currentSum + amounts[i] + FEE;
            taxes = taxes + FEE;
            require(currentSum+taxes <= msg.value, "Low balance");
            require(recipients[i] != address(0), "Recipient must be not zero address");
        }

        for (uint256 i = 0; i < recipients.length; i++) {
            
            recipients[i].transfer(amounts[i]);
        }
        payable(bank).transfer(taxes);
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
            require(amounts[i] > 0, "Value must be more than 0");
            require(recipients[i] != address(0), "Recipient must be not zero address");
            currentSum = currentSum + amounts[i];
            uint256 FEE = amounts[i].mul(percent).div(10000);
            taxes = taxes + FEE;
            require(
                currentSum+FEE <= IERC20Upgradeable(token).balanceOf(msg.sender),
                "Influence balance"
            );
            require(
                currentSum+FEE <= IERC20Upgradeable(token).allowance(msg.sender, address(this)),
                "Influence allowance"
            );
        }
        for (uint256 i = 0; i < recipients.length; i++) {
            IERC20Upgradeable(token).safeTransferFrom(msg.sender, recipients[i], amounts[i]);
        }
        IERC20Upgradeable(token).safeTransferFrom(msg.sender, bank, taxes);
    }
}
