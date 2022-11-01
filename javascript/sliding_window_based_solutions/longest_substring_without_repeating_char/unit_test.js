#!/usr/bin/env mocha

const assert = require('assert');
const { length_of_longest_substring } = require("./index.js");

const test_cases = [
	{
		input: "abcabcbb",
		output: 3,
	},
	{
		input: "bbbbb",
		output: 1,
	},
	{
		input: "pwwkew",
		output: 3,
	},
	{
		input: "     ",
		output: 1,
	},
	{
		input: "au",
		output: 2,
	}
];


describe('length of longest sub-string', function () {
	describe('test_case #1', function () {
			it('|abcabcbb| should return 3', function () {
					assert.equal(length_of_longest_substring(test_cases[0].input),test_cases[0].output);
			});
			it('|bbbbbbb| should return 1', function () {
					assert.equal(length_of_longest_substring(test_cases[1].input),test_cases[1].output);
			});
			it('|pwwkew| should return 3', function () {
					assert.equal(length_of_longest_substring(test_cases[2].input),test_cases[2].output);
			});
			it('|     | should return 1', function () {
					assert.equal(length_of_longest_substring(test_cases[3].input),test_cases[3].output);
			});
			it('|au| should return 2', function () {
					assert.equal(length_of_longest_substring(test_cases[4].input),test_cases[4].output);
			});
	});
});

