const diffx = require('../index.js');

describe('Unified API Core Features', () => {
    test('basic object comparison', () => {
        const old = { name: "Alice", age: 30 };
        const newObj = { name: "Alice", age: 31 };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expect(results[0]).toHaveProperty('diffType', 'Modified');
        expect(results[0]).toHaveProperty('path', 'age');
        expect(results[0]).toHaveProperty('oldValue');
        expect(results[0]).toHaveProperty('newValue');
    });

    test('added property', () => {
        const old = { name: "Alice" };
        const newObj = { name: "Alice", age: 30 };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expect(results[0]).toHaveProperty('diffType', 'Added');
        expect(results[0]).toHaveProperty('path', 'age');
        expect(results[0]).toHaveProperty('newValue');
    });

    test('removed property', () => {
        const old = { name: "Alice", age: 30 };
        const newObj = { name: "Alice" };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expect(results[0]).toHaveProperty('diffType', 'Removed');
        expect(results[0]).toHaveProperty('path', 'age');
        expect(results[0]).toHaveProperty('value');
    });

    test('no differences', () => {
        const old = { name: "Alice", age: 30 };
        const newObj = { name: "Alice", age: 30 };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(0);
    });

    test('nested object comparison', () => {
        const old = { user: { name: "Alice", profile: { age: 30 } } };
        const newObj = { user: { name: "Alice", profile: { age: 31 } } };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expect(results[0]).toHaveProperty('diffType', 'Modified');
        expect(results[0]).toHaveProperty('path', 'user.profile.age');
    });

    test('array comparison', () => {
        const old = { items: [1, 2, 3] };
        const newObj = { items: [1, 2, 4] };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expect(results[0]).toHaveProperty('diffType', 'Modified');
        expect(results[0]).toHaveProperty('path', 'items[2]');
    });

    test('with options', () => {
        const old = { name: "Alice", age: 30.001 };
        const newObj = { name: "Alice", age: 30.002 };
        
        const results = diffx.diff(old, newObj, { epsilon: 0.01 });
        
        expect(results).toHaveLength(0); // Should be within epsilon tolerance
    });
});