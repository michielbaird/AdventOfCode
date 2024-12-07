import sys
from dataclasses import dataclass, replace
from typing import Tuple

class TreeNode:
    def __init__(self, val, parent=None) -> None:
        self.val = val
        self.left = None
        self.right = None
        self.size = 1
        self.height = 1
        self.parent = parent
    

    
    
class AvlTree:
    def __init__(self):
        self.head = None
    
    def addNode(self, val, index=None):
        if not self.head:
            self.head = TreeNode(val)
            return self.head
        else:
            return self._add(self.head, val, index)

    def _add(self, root, val, index=None):
        #print(index, root.val)
        if index == None:
            if root.right is None:
                root.right = TreeNode(val, parent=root)
                node = root.right
            else:
                node = self._add(root.right, val)
        else:
            left_size = 0 if root.left is None else root.left.size
            if index < left_size:
                node = self._add(root.left, val, index)
            elif left_size == index:
                if root.left is None:
                    root.left = TreeNode(val, parent=root)
                    node = root.left
                else:
                    node = self._add(root.left, val)
            else:
                if root.right is None:
                    root.right = TreeNode(val, parent=root)
                    node = root.right
                else:
                    node = self._add(root.right, val, index - left_size - 1)
        self.update_size(root)
        self.balance(root)
        return node

    def getMinValueNode(self, root):
        val = root
        while val.left is not None:
            val = val.left
        return val

    def deleteNode(self, root):
        # Find the node to be deleted and remove it
        if root.left is not None and root.right is not None:
            tmp = self.getMinValueNode(root.right)
            r_p = root.parent
            r_left = root.left
            r_right = root.right

            tmp_parent = tmp.parent
            tmp_right = tmp.right


            tmp.left = r_left


            if r_right is tmp:
                tmp.right = root
            else:
                tmp.right = r_right
            
            if tmp_parent is root:
                root.parent = tmp
            else:
                root.parent = tmp_parent
                tmp_parent.left = root

            tmp.parent = r_p
            root.parent = tmp_parent
            
            root.left = None
            root.right = tmp_right
                
            if tmp.right is not None:
                tmp.right.parent = tmp
            if tmp.left is not None:
                tmp.left.parent = tmp
            if root.right is not None:
                root.right.parent = root

            if r_p is None:
                self.head = tmp
            elif r_p.left is root:
                r_p.left = tmp
            else:
                r_p.right = tmp


            #self.validate(self.head)
            self.update_size(root)
            self.update_size(tmp)
            return self.deleteNode(root)
        elif root.left is None:
            temp = root.right
        elif root.right is None:
            temp = root.left
        
        if temp is not None:
            temp.parent = root.parent

        if root.parent is None:
            self.head = temp
        elif root.parent.left is root:
            root.parent.left = temp
        else:
            root.parent.right = temp
        
        if temp is None:
            temp = root.parent

        
        while temp is not None:
            self.update_size(temp)
            self.balance(temp)
            temp = temp.parent
        root.left = None
        root.right = None
        return root

    def update_size(self, root):
        root.size = 1
        if root.left is not None:
            root.size += root.left.size
        if root.right is not None:
            root.size += root.right.size
        
        root.height = 1 + max(
            0 if root.left is None else root.left.height, 
            0 if root.right is None else root.right.height
        )
    def validate(self, root):
        if root.left is not None:
            assert root.left.parent == root
            self.validate(root.left)
        if root.right is not None:
            assert root.right.parent == root
            self.validate(root.right)


    def balance(self, root):
        bf = self.balanceFactor(root)
        # Balance the tree
        if bf > 1:
            if self.balanceFactor(root.left) >= 0:
                return self.rightRotate(root)
            else:
                root.left = self.leftRotate(root.left)
                return self.rightRotate(root)
        elif bf < -1:
            if self.balanceFactor(root.right) <= 0:
                return self.leftRotate(root)
            else:
                root.right = self.rightRotate(root.right)
                return self.leftRotate(root)
        return root
        

    def balanceFactor(self, root):
        left_h = 0 if root.left is None else root.left.height
        right_h = 0 if root.right is None else root.right.height
        return left_h - right_h

    # Function to perform left rotation
    def leftRotate(self, z):
        old_parent = z.parent

        y = z.right
        T2 = y.left

        y.left = z
        z.parent = y

        z.right = T2
        if T2 is not None:
            T2.parent = z

        y.parent = old_parent
        if old_parent is None:
            self.head = y
        elif old_parent.left is z:
            old_parent.left = y
        else:
            old_parent.right = y

        self.update_size(z)
        self.update_size(y)
        return y

    def in_order(self, root, result=None):
        if result is None:
            result = []
        if root is None:
            return result
        self.in_order(root.left, result)
        result.append(root.val)
        self.in_order(root.right, result)
        return result

    def in_order2(self, root, result=None):
        if result is None:
            result = []
        if root is None:
            return result
        self.in_order2(root.left, result)
        result.append(root)
        self.in_order2(root.right, result)
        return result

    # Function to perform right rotation
    def rightRotate(self, z):
        old_parent = z.parent

        y = z.left
        T3 = y.right

        y.right = z
        z.parent = y

        z.left = T3
        if T3 is not None:
            T3.parent = z

        y.parent = old_parent
        if old_parent is None:
            self.head = y
        elif old_parent.left is z:
            old_parent.left = y
        else:
            old_parent.right = y

        self.update_size(z)
        self.update_size(y)
        return y

    def getIndex(self, root):
        index = 0 if root.left is None else root.left.size
        tmp = root
        while tmp.parent is not None:
            if tmp.parent.right == tmp:
                index += 1 + (0 if tmp.parent.left is None else tmp.parent.left.size)
            tmp = tmp.parent
        return index
        

    def to_print(self, root, raw=None, level=0):
        if raw is None:
            raw=[]
        if root is None:
            raw.append(" "*level + "None")
            return raw
        raw.append( f"TreeNode({root.val}, size, {root.size}")
        raw.append(" "*level + "L: " + "\n".join(self.to_print(root.left, [], level+1)))
        raw.append(" "*level + "R: " + "\n".join(self.to_print(root.right, [], level+1)))
        raw.append(" "*level + ")")
        return raw
    
    def __repr__(self) -> str:
        return "\n".join(self.to_print(self.head))
    

def main():
    values = [int(line) for line in sys.stdin.read().split("\n")]
    #values =  [1, 2, -3, 3, -2, 0, 4]
    #print(values)
    tree = AvlTree()
    nodes = []
    for v in values:
        nodes.append(tree.addNode(v))

    N = len(nodes)
    for node in nodes:
        shift = node.val
        index = tree.getIndex(node)
        new_index = index + shift
        new_index %= (N - 1)
        if new_index == 0 and shift < 0:
            new_index = N - 1
        tree.deleteNode(node)
        tree.addNode(shift, new_index)



    #print(tree)
    #print([tree.getIndex(node) for node in nodes])
    li = tree.in_order(tree.head)
    index = li.index(0)

    to_sum = [li[(index+ 1000) % N], li[(index + 2000) % N], li[(index+ 3000) % N]]
    print(to_sum)
    print(sum(to_sum))

    tree = AvlTree()
    key = 811589153
    
    nodes = [tree.addNode(val*key) for val in values]
    for round in range(10):
        print(round)
        new_nodes = []
        for node in nodes:
            shift = node.val
            index = tree.getIndex(node)
            new_index = index + shift
            new_index %= (N - 1)
            if new_index == 0 and shift < 0:
                new_index = N - 1
            tree.deleteNode(node)
            new_nodes.append(tree.addNode(shift, new_index))
        nodes = new_nodes
        #nodes = tree.in_order2(tree.head)
        #print(nodes)
    
    
    li = tree.in_order(tree.head)
    #print(li)
    index = li.index(0)

    to_sum = [li[(index+ 1000) % N], li[(index + 2000) % N], li[(index+ 3000) % N]]
    print(to_sum)
    print(sum(to_sum))



    

if __name__ == "__main__":
    main()