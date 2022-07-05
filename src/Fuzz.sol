pragma solidity ^0.8.15;

import "solmate/utils/FixedPointMathLib.sol";

contract Fuzz {
    function name() external pure returns (string memory) {
        return "Fuzz";
    }

    function add(uint a, uint b) external pure returns (uint) {
        return a + b;
    }

    function ln(int a) external pure returns (int) {
        require(a > 0);
        return FixedPointMathLib.lnWad(a);
    }

    function exp(int a) external pure returns (int) {
        return FixedPointMathLib.expWad(a);
    }

}
