import math


class Solution:
    def divide(self, dividend: int, divisor: int) -> int:
        # convert all in absolute values
        # while remainder greater than or equal divisor
        # decrement the divisor from the dividend
        # check if still there is remainder.
        # Continues in loop if true
        # returning according to divisor and dividend signs

        isNeg = (dividend > 0) ^ (divisor > 0)

        dividend = abs(dividend)
        divisor = abs(divisor)

        if divisor == 1:
            if isNeg:
                dividend = -dividend

            return min(max(-2147483648, dividend), 2147483647)

        quotient = 0

        while (dividend >= divisor):
            dividend -= divisor
            quotient += 1

        if isNeg:
            quotient = -quotient

        return min(max(-2147483648, quotient), 2147483647)


s = Solution()
print(3 == s.divide(10, 3))
print(-2 == s.divide(7, -3))
print(-2 == s.divide(-7, 3))
print(5 == s.divide(-10, -2))
print(s.divide(10, 2))
