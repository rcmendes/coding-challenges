import math

class Solution:
    def getMinMediumMaxRoman(self, decimal_order:int) -> (str, str, str):
        if decimal_order >= 1000:
            return ("M", "M", "M")
        elif 100 <= decimal_order < 1000:
            return ("C", "D", "M")
        elif 10 <= decimal_order < 100:
            return ("X", "L", "C")
        else:
            return ("I", "V", "X")        

    def intToRoman(self, num: int) -> str:
        num_as_str = str(num)
        
        length = len(num_as_str)
        result = ""
        for i in range(0, length):
            digit = int(num_as_str[i])            
            
            decimal_slot = int(math.pow(10, length-i -1))
            
            romanNumbers = self.getMinMediumMaxRoman(decimal_slot)
            
            if digit <= 3:
                result += romanNumbers[0] *digit
            elif digit ==4:
                result += romanNumbers[0] + romanNumbers[1] 
            elif digit == 5:
                result += romanNumbers[1] 
            elif 5 < digit <=8:
                result += romanNumbers[1]  + romanNumbers[0] * (digit - 5)
            elif digit ==9:
                result += romanNumbers[0] + romanNumbers[2] 

        return result

    def intToRoman2(self, num: int) -> str:
        num_as_str = str(num)
        
        length = len(num_as_str)
        result = ""
        for i in range(0, length):
            digit = int(num_as_str[i])            
            # print(digit)
            decimal_slot = int(math.pow(10, length-i -1))
            print(digit * decimal_slot)

            if decimal_slot >= 1000:
                if digit <= 3:
                    result += "M"*digit
                else:
                    print("ERROR!!!")

            elif 100 <= decimal_slot < 1000:
                if digit <= 3:
                    result += "C"*digit
                elif digit ==4:
                    result += "CD"
                elif digit == 5:
                    result += "D"
                elif 5 < digit <=8:
                    result += "D" + "C"* (digit - 5)
                elif digit ==9:
                    result += "CM"
            elif 10 <= decimal_slot < 100:
                if digit <= 3:
                    result += "X"*digit
                elif digit ==4:
                    result += "XL"
                elif digit == 5:
                    result += "L"
                elif 5 < digit <=8:
                    result += "L" + "X"* (digit - 5)
                elif digit ==9:
                    result += "XC"
            else:
                if digit <= 3:
                    result += "I"*digit
                elif digit ==4:
                    result += "IV"
                elif digit == 5:
                    result += "V"
                elif 5 < digit <=8:
                    result += "V" + "I"* (digit - 5)
                elif digit ==9:
                    result += "IX"

        return result    

s = Solution()
print(s.intToRoman(4))
print(s.intToRoman(1980))