import math

# Definition for singly-linked list.


class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

    def __str__(self):
        result = str(self.val)

        next = self.next
        while next:
            result += " -> " + str(next.val)
            next = next.next

        return result


class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode, accum=0) -> ListNode:
        node1 = l1
        node2 = l2

        result = None
        result_last_node = None
        while node1:
            node2_val = 0
            if node2:
                node2_val = node2.val

            sum = node1.val + node2_val + accum

            digit = sum % 10
            accum = math.floor(sum / 10)

            if not result:
                result = ListNode(digit)
                result_last_node = result
            else:
                result_last_node.next = ListNode(digit)
                result_last_node = result_last_node.next

            node1 = node1.next

            if node2:
                node2 = node2.next

        if node2:
            result_last_node.next = self.addTwoNumbers(
                node2, result_last_node.next, accum)
        elif accum > 0:
            result_last_node.next = ListNode(accum)

        return result


s = Solution()

# l1 = ListNode(1)
# l1.next = ListNode(2)
# l1.next.next = ListNode(7)

# l2 = ListNode(4)
# l2.next = ListNode(9)
# l2.next.next = ListNode(9)

# print(s.addTwoNumbers(l1, l2))


l3 = ListNode(1)

l4 = ListNode(9)
l4.next = ListNode(9)

print(s.addTwoNumbers(l3, l4))
