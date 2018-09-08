class Array
  def sum
    inject(0) { |result, el| result + el }
  end

  def mean
    if sum % size == 0
      sum / size
    else
      sum * 1.0 / size
    end
  end

  def iml
    (length - 1) / 2
  end

  def imr
    length / 2
  end

  def ml
    iml == 0 ? self[iml] : self[iml - 1]
  end

  def mr
    imr == length - 1 ? self[imr] : self[imr + 1]
  end

  def median
    if length % 2 == 1
      self[length / 2]
    else
      (self[length / 2] + self[length / 2 - 1]) / 2.0
    end
  end
end

# @param {Integer[]} nums1
# @param {Integer[]} nums2
# @return {Float}
def find_median_sorted_arrays(nums1, nums2)
    return nums2.median if nums1.length == 0
    return nums1.median if nums2.length == 0
    return [nums1[0], nums2[0]].mean if nums1.length == 1 && nums2.length == 1

    nums = [0, nums1, nums2]
    if nums[1].ml < nums[2].ml
      x = 1
    else
      x = 2
    end
    if nums[1].mr > nums[2].mr
      y = 1
    else
      y = 2
    end
    return nums[x].ml if nums[x].ml == nums[y].mr

    l1 = [nums[x].iml, 1].max
    l2 = [nums[y].length - 1 - nums[y].imr, 1].max
    l = [l1, l2].min
    nums[x] = nums[x][l, nums[x].length - l]
    nums[y] = nums[y][0, nums[y].length - l]
    return find_median_sorted_arrays(nums[1], nums[2])
end
