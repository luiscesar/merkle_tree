# Balanced Merkle Tree

## Indroduction
A Merkle Tree is a concept often used in Blockchains.
It is a binary tree where each leaf node represents the hash of some interesting data
and each internal node is the hash of the concatenated contents of its two children.
Merkle Trees often record groups of transactions, and the roots are published widely to
serve as summaries of all recognised transactions on a given date.

By construction, the tree's root is a hash of all its leaves organised in a specific order.
Since hash functions are hard to reverse, it is unfeasible to create a tree with a specific
root if we don't know the inputs to it.
We can prove a transaction happened before a certain date by showing it was a leaf of Merkle Tree
that has already been published.

Merkle Trees provide an efficient way to prove this inclusion in the tree.
It is enough to show a path of neighbour-nodes from the leaf to the root.
That is, a list that includes the sibling of the leaf node, then the sibling of its parent and so on until the root is reached.

Here a balanced Merkle tree implementation is shown.


