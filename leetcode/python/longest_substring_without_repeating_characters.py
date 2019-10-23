class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        read_map = []
        max = 0
        for c in s:
            index = -1
            try:
                index = read_map.index(c)
            except ValueError:
                index = -1

            if index >= 0:                
                read_map = read_map[index+1:]
                
            read_map.append(c)
            if max < len(read_map):
                max = len(read_map)
        return max

s = Solution()

print(3 == s.lengthOfLongestSubstring("abcabcbb"))
print(1 == s.lengthOfLongestSubstring("bbbbb"))
print(3 == s.lengthOfLongestSubstring("pwwkew"))
print(3 == s.lengthOfLongestSubstring("dvdf"))