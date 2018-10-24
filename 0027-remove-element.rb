# @param {Integer[]} nums
# @param {Integer} val
# @return {Integer}
def remove_element(nums, val)
  nums.select! {|item| val != item}
  return nums.length
end
