class Node:
    def __init__(self, char=None, freq=0):
        self.char = char
        self.freq = freq
        self.left = None
        self.right = None

nodes = []

def calculate_frequencies(word):
    frequencies = {}
    for char in word:
        if char not in frequencies:
            freq = word.count(char)
            frequencies[char] = freq
            nodes.append(Node(char, freq))

def build_huffman_tree():
    while len(nodes) > 1:
        nodes.sort(key=lambda x: x.freq)
        left = nodes.pop(0)
        right = nodes.pop(0)
        
        merged = Node(freq=left.freq + right.freq)
        merged.left = left
        merged.right = right
        
        nodes.append(merged)

    return nodes[0]

def generate_huffman_codes(node, current_code, codes):
    if node is None:
        return

    if node.char is not None:
        codes[node.char] = current_code

    generate_huffman_codes(node.left, current_code + '0', codes)
    generate_huffman_codes(node.right, current_code + '1', codes)

def huffman_encoding(word):
    global nodes
    nodes = []
    calculate_frequencies(word)
    root = build_huffman_tree()
    codes = {}
    generate_huffman_codes(root, '', codes)
    return codes


if __name__ == '__main__':
    with open("moby_dick2.txt", 'r', encoding="UTF-8") as book:
        text = book.read()
        huff = huffman_encoding(text)
        print(f"Number of chars: {len(text)}")
        print()
        print("Codes:")
        print(huff)
        print()
        n_bits = 0
        for ch in text:
            n_bits += len(huff[ch])
        print(f"Uncompressed size in bits: {len(text) * 8}")
        print(f"Compressed size in bits: {n_bits}")
        print(f"Ratio: {(len(text) * 8) / n_bits}")