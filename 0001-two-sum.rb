# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer[]}
def two_sum(nums, target)
  hash = Hash.new
  nums.each.with_index { |x,i|
    y = target - x
    if hash.include?(y)
      return [hash[y], i]
    end
    hash[x] = i
  }
end
