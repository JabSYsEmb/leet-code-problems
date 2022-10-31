#!/usr/bin/env node

const length_of_longest_substring = (str) => {
	if (str.length < 2) return str.length;

	let start = 0;
	let   end = 1;
	let   win = (end - start);

	while( end <= str.length & win !== str.length){
		if(win < (end - start)) win = end - start;
		if( !str.slice(start, end).includes(str[end]) ) {
			end += 1;
			continue;
		}
		start += 1;
	}

	return win;
}

module.exports = {
	length_of_longest_substring,
}
