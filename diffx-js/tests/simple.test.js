const diffx = require('../index.js');

describe('Simple Test', () => {
    test('basic diff works', () => {
        const old = { name: "Alice", age: 30 };
        const newObj = { name: "Alice", age: 31 };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expect(results[0]).toHaveProperty('diffType', 'Modified');
        expect(results[0]).toHaveProperty('path', 'age');
    });

    test('no changes', () => {
        const old = { name: "Alice", age: 30 };
        const newObj = { name: "Alice", age: 30 };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(0);
    });
});