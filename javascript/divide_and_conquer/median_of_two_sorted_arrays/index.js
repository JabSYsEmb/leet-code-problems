
// [-2,0,5,8]
// [1,2,6,9]
// [-2, 0, 1, 2, 5, 6, 8, 9]

const find_median_sorted_arrays = (nums1, nums2) => {

	const arr = [];

	while ( nums1.length && nums2.length ) {
		const temp = nums2.shift();
		while ( nums1[0] < temp ){
			arr.push(nums1.shift());
		}
		arr.push(temp);
	}

	if(nums1.length !== 0) arr.push(...nums1);
	if(nums2.length !== 0) arr.push(...nums2);

	if ( arr.length % 2 === 0 ){
		return ( arr[arr.length/2 - 1] + arr[arr.length/2] )/2;
	}else{
		return ( arr[Math.floor(arr.length/2)] );
	}
}

module.exports = {
	find_median_sorted_arrays,
}
