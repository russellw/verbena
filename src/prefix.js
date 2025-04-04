#!/usr/bin/env node
'use strict';

const assert = require('assert');

const abs=Math.abs;
const min=Math.min;
const max=Math.max;
const sqrt=Math.sqrt;

function _get(a,k){
	if(a instanceof Map)
		return  a.get(k);
	return a[k];
}

function _set(a,k,x){
	if(a instanceof Map)
		a.set(k,x);
	else
		a[k]=x;
	return x
}

function len(a){
	if(a instanceof Map)
		return  a.size;
	return a.length
}

function _prin(a){
	process.stdout.write(a.toString())
}

function str(a){
	return a.toString()
}
