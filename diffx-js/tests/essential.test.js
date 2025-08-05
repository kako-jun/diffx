const diffx = require('../index.js');

describe('diffx-js Essential Tests', () => {
    test('basic modification', () => {
        const old = { name: "Alice", age: 30 };
        const newObj = { name: "Alice", age: 31 };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expect(results[0].diffType).toBe('Modified');
        expect(results[0].path).toBe('age');
    });

    test('add and remove', () => {
        const old = { name: "Alice", age: 30 };
        const newObj = { name: "Alice", city: "Tokyo" };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(2);
        
        const types = results.map(r => r.diffType).sort();
        expect(types).toEqual(['Added', 'Removed']);
    });

    test('nested objects', () => {
        const old = { user: { profile: { age: 30 } } };
        const newObj = { user: { profile: { age: 31 } } };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expect(results[0].path).toBe('user.profile.age');
    });

    test('arrays', () => {
        const old = { items: [1, 2, 3] };
        const newObj = { items: [1, 4, 3] };
        
        const results = diffx.diff(old, newObj);
        
        expect(results).toHaveLength(1);
        expect(results[0].path).toBe('items[1]');
    });

    test('epsilon option', () => {
        const old = { value: 1.001 };
        const newObj = { value: 1.002 };
        
        const withoutEpsilon = diffx.diff(old, newObj);
        const withEpsilon = diffx.diff(old, newObj, { epsilon: 0.01 });
        
        expect(withoutEpsilon).toHaveLength(1);
        expect(withEpsilon).toHaveLength(0);
    });

    test('arrayIdKey option', () => {
        const old = { users: [{ id: 1, name: "Alice" }] };
        const newObj = { users: [{ id: 1, name: "Bob" }] };
        
        const results = diffx.diff(old, newObj, { arrayIdKey: 'id' });
        
        expect(results).toHaveLength(1);
        expect(results[0].path).toMatch(/name/);
    });

    test('ignoreKeysRegex option', () => {
        const old = { name: "Alice", timestamp: "2023-01-01" };
        const newObj = { name: "Alice", timestamp: "2023-01-02" };
        
        const results = diffx.diff(old, newObj, { ignoreKeysRegex: "timestamp" });
        
        expect(results).toHaveLength(0);
    });

    test('no differences', () => {
        const data = { name: "Alice", age: 30 };
        
        const results = diffx.diff(data, data);
        
        expect(results).toHaveLength(0);
    });
});