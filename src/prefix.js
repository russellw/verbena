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

function eq(a, b) {
  // Check if both values are the same reference
  if (a === b) return true;
  
  // If either is null/undefined or not an object, they're not equal
  if (a == null || b == null || typeof a !== 'object' || typeof b !== 'object') return false;
  
  // Check if they are arrays
  const aIsArray = Array.isArray(a);
  const bIsArray = Array.isArray(b);
  
  // Both should be arrays or both should be objects
  if (aIsArray !== bIsArray) return false;
  
  if (aIsArray) {
    // Check array length
    if (a.length !== b.length) return false;
    
    // Compare each element
    for (let i = 0; i < a.length; i++) {
      if (!eq(a[i], b[i])) return false;
    }
    return true;
  } else {
    // For objects (including plain objects, Maps, Sets, etc.)
    const keysA = Object.keys(a);
    const keysB = Object.keys(b);
    
    // Check if they have the same number of properties
    if (keysA.length !== keysB.length) return false;
    
    // Check if all properties in a exist in b with the same values
    for (const key of keysA) {
      if (!Object.prototype.hasOwnProperty.call(b, key) || !eq(a[key], b[key])) {
        return false;
      }
    }
    return true;
  }
}
