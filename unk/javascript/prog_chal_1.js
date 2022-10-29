function Person({
	fname = 'John',
	lname = 'Doe',
	age = 1,
}) {
	return { fname, lname, age }
}

// EXAMPLE OUTPUT
const person = Person({}); // { fname: 'John', lname: 'Doe', age: 1 }
const person2 = Person({ age: 21, fname: "Tyler" }); // { age: 21, fname: 'Tyler', lname: 'Doe' }
console.log(person)
console.log(person2)
