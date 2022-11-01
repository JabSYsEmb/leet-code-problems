#!/usr/bin/env mocha


const assert = require('assert');
const { find_median_sorted_arrays } = require("./index.js");

const test_cases = [
	{
		input: [[1,3], [2]],
		output: 2.00000,
	},
	{
		input: [[1,2], [3,4]],
		output: 2.50000,
	},
	{
		input: [[1,2], []],
		output: 1.50000,
	},
	{
		input: [[0], [0]],
		output: 0.00000,
	},
	{
		input: [[1,3,4,5,7], [0]],
		output: 3.5,
	},
	{
		input: [[7,9,10,15,17], [1,3,4]],
		output: 8,
	}
];


describe('length of longest sub-string', function () {
	describe('test_case #1', function () {
			it('[[1,3], [2]] should return 2.0000', function () {
					assert.equal(find_median_sorted_arrays(...test_cases[0].input),test_cases[0].output);
			});
			it('[[1,2], [3,4]] should return 2.5000', function () {
					assert.equal(find_median_sorted_arrays(...test_cases[1].input),test_cases[1].output);
			});
			it('[[1,2],[]] should return 1.5000', function () {
					assert.equal(find_median_sorted_arrays(...test_cases[2].input),test_cases[2].output);
			});
			it('[[0],[0]] should return 0.0000', function () {
					assert.equal(find_median_sorted_arrays(...test_cases[3].input),test_cases[3].output);
			});
			it('[[1,3,4,5,7],[0]] should return 3.5000', function () {
					assert.equal(find_median_sorted_arrays(...test_cases[4].input),test_cases[4].output);
			});
			it('[[[7,9,10,15,17], [1,3,4]]] should return 8.0000', function () {
					assert.equal(find_median_sorted_arrays(...test_cases[5].input),test_cases[5].output);
			});
	});
});

