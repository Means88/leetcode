# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val)
#         @val = val
#         @next = nil
#     end
# end

# @param {ListNode} l1
# @param {ListNode} l2
# @return {ListNode}
def add_two_numbers(l1, l2)
    flag = 0
    sum = ListNode.new(0)
    current = sum
    while l1 && l2
      x = l1.val + l2.val + flag
      current.next = ListNode.new(x % 10)
      flag = x / 10
      current = current.next
      l1 = l1.next
      l2 = l2.next
    end
    rest = l1 || l2
    while rest
      x = rest.val + flag
      current.next = ListNode.new(x % 10)
      flag = x / 10
      current = current.next
      rest = rest.next
    end
    if flag > 0
      current.next = ListNode.new(1)
    end
    return sum.next
end
