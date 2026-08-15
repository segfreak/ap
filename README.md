<h1 align="center">ap</h1>

<p align="center">
  <strong>Fast arbitrary-precision fixed-width integers for Rust</strong>
</p>

<p align="center">
  A lightweight integer library designed for compilers, interpreters, virtual machines, and systems software.
</p>

<hr>

<h2>Overview</h2>

<p>
  <code>ap</code> is an arbitrary-precision integer library for Rust focused on
  <strong>fixed-width integer semantics</strong>.
</p>

<p>
  It is designed primarily for compiler infrastructure and provides an
  <a href="https://llvm.org/docs/ProgrammersManual.html#the-apint-class">LLVM APInt</a>-like
  programming model without depending on LLVM.
</p>

<pre><code>use ap::ApInt;

let a = ApInt::new(32, 100);
let b = ApInt::new(32, 50);

let sum = a + b;

assert_eq!(sum.to_u32_lossy(), 150);</code></pre>

<h2>Features</h2>

<ul>
  <li>Arbitrary positive bit widths</li>
  <li>Fixed-width two's-complement semantics</li>
  <li>64-bit little-endian limbs</li>
  <li>Optional <code>SmallVec</code> storage</li>
  <li>Optional Serde support</li>
  <li>Signed and unsigned arithmetic</li>
  <li>Signed and unsigned comparisons</li>
  <li>Bitwise operations</li>
  <li>Logical and arithmetic shifts</li>
  <li>Zero extension</li>
  <li>Sign extension</li>
  <li>Truncation</li>
  <li>Signed and unsigned division and remainder</li>
  <li>Native integer conversions</li>
  <li><code>std::ops</code> integration</li>
  <li>Compiler-oriented API</li>
  <li>No dependency on LLVM</li>
</ul>

<h2>Installation</h2>

<pre><code>[dependencies]
ap = "0.1"</code></pre>

<h3>SmallVec</h3>

<p>
  Enable the <code>smallvec</code> feature to avoid heap allocations for small
  integers.
</p>

<pre><code>[dependencies]
ap = { version = "0.1", features = ["smallvec"] }</code></pre>

<h3>Serde</h3>

<pre><code>[dependencies]
ap = { version = "0.1", features = ["serde"] }</code></pre>

<p>Both features can be enabled together:</p>

<pre><code>[dependencies]
ap = { version = "0.1", features = ["smallvec", "serde"] }</code></pre>

<h2>Basic Usage</h2>

<h3>Construction</h3>

<pre><code>use ap::ApInt;

let value = ApInt::new(32, 123);

let zero = ApInt::zero(32);
let one = ApInt::one(32);</code></pre>

<p>
  Values exceeding the requested width are truncated.
</p>

<pre><code>let value = ApInt::new(8, 0x1234);

assert_eq!(value.to_u8_lossy(), 0x34);</code></pre>

<h2>Arbitrary Widths</h2>

<p>
  <code>ApInt</code> is not limited to native machine integer sizes.
</p>

<pre><code>let value = ApInt::new(256, 123);

assert_eq!(value.width(), 256);</code></pre>

<p>Any positive bit width can be represented:</p>

<pre><code>i1
i7
i8
i13
i32
i64
i128
i256
i1024
i4096
...</code></pre>

<h2>Signed Integers</h2>

<p>
  <code>ApInt</code> stores values as fixed-width bit patterns. The same bit
  pattern can be interpreted as either signed or unsigned.
</p>

<pre><code>let value = ApInt::new(8, 0xff);

assert!(value.is_negative());
assert!(value.slt(&amp;ApInt::zero(8)));</code></pre>

<p>For an 8-bit integer:</p>

<pre><code>0xff = 255 unsigned
0xff = -1 signed</code></pre>

<h2>Arithmetic</h2>

<pre><code>let a = ApInt::new(32, 100);
let b = ApInt::new(32, 25);

let add = &amp;a + &amp;b;
let sub = &amp;a - &amp;b;
let mul = &amp;a * &amp;b;</code></pre>

<p>
  Arithmetic uses fixed-width modular semantics.
</p>

<pre><code>let a = ApInt::new(8, 255);
let b = ApInt::new(8, 1);

let result = &amp;a + &amp;b;

assert_eq!(result.to_u8_lossy(), 0);</code></pre>

<h2>Division</h2>

<p>
  Both signed and unsigned division are supported.
</p>

<h3>Unsigned</h3>

<pre><code>let a = ApInt::new(64, 100);
let b = ApInt::new(64, 3);

let quotient = a.udiv(&amp;b);
let remainder = a.urem(&amp;b);</code></pre>

<h3>Signed</h3>

<pre><code>let a = ApInt::new(8, 0xf6);
let b = ApInt::new(8, 2);

let quotient = a.sdiv(&amp;b);
let remainder = a.srem(&amp;b);</code></pre>

<p>
  Signed division truncates toward zero, matching C-like and LLVM-style
  integer division semantics.
</p>

<h2>Bitwise Operations</h2>

<pre><code>let a = ApInt::new(8, 0b1010);
let b = ApInt::new(8, 0b1100);

let and = a.bitand(&amp;b);
let or  = a.bitor(&amp;b);
let xor = a.bitxor(&amp;b);
let not = a.not();</code></pre>

<p>
  The corresponding Rust operators are also implemented.
</p>

<pre><code>let result = (&amp;a &amp; &amp;b) | &amp;a;</code></pre>

<h2>Shifts</h2>

<pre><code>let value = ApInt::new(32, 0x100);

let left = value.shl(4);
let right = value.lshr(4);</code></pre>

<p>
  Arithmetic right shift preserves the sign bit.
</p>

<pre><code>let value = ApInt::new(8, 0xff);

let result = value.ashr(1);

assert_eq!(result.to_u8_lossy(), 0xff);</code></pre>

<h2>Width Conversion</h2>

<h3>Zero Extension</h3>

<pre><code>let value = ApInt::new(8, 0xff);
let extended = value.zext(32);</code></pre>

<h3>Sign Extension</h3>

<pre><code>let value = ApInt::new(8, 0x80);
let extended = value.sext(32);

assert_eq!(extended.to_u32_lossy(), 0xffff_ff80);</code></pre>

<h3>Truncation</h3>

<pre><code>let value = ApInt::new(64, 0x1234);
let truncated = value.trunc(8);

assert_eq!(truncated.to_u8_lossy(), 0x34);</code></pre>

<h2>Comparisons</h2>

<h3>Unsigned</h3>

<pre><code>a.ult(&amp;b);
a.ule(&amp;b);
a.ugt(&amp;b);
a.uge(&amp;b);</code></pre>

<h3>Signed</h3>

<pre><code>a.slt(&amp;b);
a.sle(&amp;b);
a.sgt(&amp;b);
a.sge(&amp;b);</code></pre>

<h2>Representation</h2>

<p>
  Values are represented as 64-bit limbs in little-endian order.
</p>

<pre><code>Most significant                         Least significant

+----------------+----------------+----------------+
|    limb[2]     |    limb[1]     |    limb[0]     |
+----------------+----------------+----------------+
                                              </code></pre>

<p>
  For a 130-bit integer:
</p>

<pre><code>limb[0] = bits 0..63
limb[1] = bits 64..127
limb[2] = bits 128..129</code></pre>

<p>
  Unused bits in the most significant limb are always cleared.
</p>

<h2>Storage</h2>

<p>Without the <code>smallvec</code> feature:</p>

<pre><code>type Limbs = Vec&lt;u64&gt;;</code></pre>

<p>With the <code>smallvec</code> feature:</p>

<pre><code>type Limbs = SmallVec&lt;[u64; 1]&gt;;</code></pre>

<p>
  This allows small integers to remain entirely inline and avoid heap
  allocation.
</p>

<p>
  This is particularly useful for compiler workloads where
  <code>i1</code>, <code>i8</code>, <code>i16</code>, <code>i32</code>, and
  <code>i64</code> values are extremely common.
</p>

<h2>Compiler-Oriented Design</h2>

<p>
  <code>ap</code> is designed for workloads commonly found in compiler
  infrastructure:
</p>

<ul>
  <li>IR integer constants</li>
  <li>Constant folding</li>
  <li>SCCP</li>
  <li>GVN</li>
  <li>Constant propagation</li>
  <li>Instruction selection</li>
  <li>Integer range analysis</li>
  <li>Assembler and machine-code tooling</li>
  <li>Virtual machines</li>
</ul>

<p>
  The fixed-width representation maps naturally to compiler integer types.
</p>

<h2>Performance</h2>

<p>
  The implementation operates directly on 64-bit limbs and is optimized around
  common compiler workloads.
</p>

<ul>
  <li>Small integers can avoid heap allocation with <code>SmallVec</code></li>
  <li>64-bit limbs provide efficient arithmetic</li>
  <li>Common operations operate directly on limb arrays</li>
  <li>Fixed-width semantics avoid dynamic precision management</li>
</ul>

<p>
  The implementation currently uses straightforward limb-based algorithms,
  with further optimization possible for multiplication and division.
</p>

<h2>Design Goals</h2>

<ul>
  <li><strong>Fast</strong> — minimize overhead for common small-width integers.</li>
  <li><strong>Small</strong> — keep the implementation lightweight.</li>
  <li><strong>Predictable</strong> — provide explicit fixed-width semantics.</li>
  <li><strong>Compiler-friendly</strong> — model integer types used by compiler IRs.</li>
  <li><strong>Portable</strong> — implemented entirely in Rust.</li>
  <li><strong>Independent</strong> — no dependency on LLVM.</li>
</ul>

<h2>Non-Goals</h2>

<p>
  <code>ap</code> is not intended to be a general-purpose replacement for
  arbitrary-precision mathematical integer libraries such as
  <code>num-bigint</code>.
</p>

<p>
  The focus is fixed-width compiler integers:
</p>

<pre><code>ApInt&lt;32&gt; + ApInt&lt;32&gt; -&gt; ApInt&lt;32&gt;</code></pre>

<p>
  rather than dynamically growing mathematical integers.
</p>

<h2>Inspiration</h2>

<p>
  The API and semantics are inspired by LLVM's
  <code>APInt</code>, while <code>ap</code> remains an independent Rust
  implementation.
</p>

<p>
  The goal is to provide efficient fixed-width integer semantics for compiler
  projects without requiring an LLVM dependency.
</p>

<h2>License</h2>

<p>
  Licensed under either the MIT License
</p>

<hr>

<p align="center">
  <strong>Built for compilers. Designed for fixed-width integers.</strong>
</p>