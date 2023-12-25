# Advent of Code 2023

- https://adventofcode.com/
- https://github.com/livexia/advent-of-code-2023

[toc]

## Rust Hints

- https://github.com/dtolnay/anyhow This library provides `[anyhow::Error](https://docs.rs/anyhow/1.0/anyhow/struct.Error.html)`, a trait object based error type for easy idiomatic error handling in Rust applications.
- `flat_map(|n| n.parse())` 会忽略 `Err` 保留 `Ok` 中的结果。

## Things to Learn

- Dynamic programming
- Dijkstra's algorithm
- Shortest path problem
- Cycle detection
    - Floyd's tortoise and hare
- Shoelace formula
- Pick’s theorem
- Purne graph
    - Edge contraction
- Longest path problem
- Linear algebra

## Day 1

Part 2 一些特别的思路

- 通过替换 zero 为 zero0zero 将字母串替换为包含数字字符的新字符串，巧妙的实现字母串转换为数字字符，同时又避免因为替换而导致其他可能的字母串被修改 https://old.reddit.com/r/adventofcode/comments/1883ibu/2023_day_1_solutions/kbikddg/ advent-of-code-2023
Advent of Code 2023

## Day 2

所耗时间均在对输入的处理上，解决问题的思路直截了当。

## Day 3

依旧需要小心的处理输入，涉及到二维的矩阵，往往有两种方法：一是利用二维 Vec 进行表示，二则是利用 HashMap 进行表示。如果利用 Vec 那么在涉及到对周边节点遍历时，在 Rust 中要注意对 usize 数据类型可能的 underflow ，利用 HashMap 则可以利用 i32 作为矩阵的坐标数据类型。

在具体的算法上，我选择利用深度优先搜索，进行解决，具体思路如下：

1. 从矩阵的右下角开始进行深度优先搜索，从右到左，从上到下的顺序分别进行dfs。
2. 如果当前元素是数字，那么这个数字必然是一个数字的组成部分，同时也是这个数字的最低位，这样的搜索顺序可以方便的对最终结果进行结合。
3. 同样的如果当前元素是数字，那么可以对这个数字的邻接元素（8个）进行判断，如果其中存在一个符号，设标识，表示当前数字同一个符号相邻。
4. 除了对邻接元素进行判断，同时要对当前元素的左侧元素进行递归的深度优先搜索，深度优先返回得到整个数字除了最低位的部分，对这个结果乘 10 加上当前元素的数字，即是完整的数字。
5. 同样的一个数字任意位的邻接是符号，则整个数字都与符号邻接，于是需要在深度优先的搜索过程中保留标识，每次都对标识进行或运算。
6. 在深度优先搜索中引入 visited ，确保元素不被重复搜索。
7. 总结：深度优先搜索是从一个数字开始，在一行中向左继续搜索，搜索过程中取得一个完整的数字，并确认整个数字是否存在部分与符号邻接。
8. 第一部分的方法即是如此，第二部分则稍有不同，但整体思路是一致的，在深度优先搜索中，要判断数字的组成部分是否与齿轮符号邻接，并保留数字每个组成部分对应的齿轮符号坐标（一个齿轮可能与多个数字邻接，同时一个数字也可能与多个符号邻接）。
9. 在每一次深度优先搜索后，就得到了数字与邻接齿轮的对应关系，在这个基础上再对问题进行解答即可。

## Day 4

这个问题的输入并不复杂，每一行输入包含了两个数组，第一部分要求计算第二个数组中的数字在第一个数组中出现的次数，直接暴力匹配实现即可。第二部分需要在这个基础上再进一步的计算，在此以示例输入为例进行阐述：

- 依旧是对输入进行暴力匹配，计算每一行输入中第二个数组中的数字在第一个数组中出现的次数，对于示例输入可得 `[4, 2, 2, 1, 0, 0]`
- 当前所有的 Card 数量均为 `[1, 1, 1, 1, 1, 1]`
- 根据题意，因为 Card 1 中重复的数字有 4 个，则会获得 Card 2、3、4、5 各一张
- 此时各 Card 数量为 `[1, 2, 2, 2, 2, 1]`
- 因为 Card 2 有两张，同时 Card 2 中重复的数字有 2 个，则会 Card 3、4 各两张
- 此时各 Card 数量为 `[1, 2, 2 + 2, 2 + 2, 2, 1]`
- 依次类推
- 刮完所有的 Card 3后，Card 数量为 `[1, 2, 2 + 2, 2 + 2 + 4, 2 + 4, 1]`
- 刮完所有的 Card 4后，Card 数量为 `[1, 2, 2 + 2, 2 + 2 + 4, 2 + 4 + 8, 1]`
- 最后的 Crad 总数即为 `1 + 2 + 4 + 8 + 14 + 1 = 30`

****************************************性能优化？****************************************

- 避免暴力匹配，选用更好的匹配算法，计算两个数组的重叠元素个数。**实际上在这个问题中，需要匹配的两个数组大小都较小，暴力匹配并不会影响性能。**
    - https://afteracademy.com/blog/find-the-intersection-of-two-unsorted-arrays/
    - 对其中一个数组进行排序，在二分查找另一个数组的元素
    - 对两个数组都进行排序，双指针进行对比 [https://afteracademy.com/blog/find-the-intersection-of-two-unsorted-arrays/#:~:text=Return answer list.-,Solution Visualization,-Pseudo-Code](https://afteracademy.com/blog/find-the-intersection-of-two-unsorted-arrays/#:~:text=Return%20answer%20list.-,Solution%20Visualization,-Pseudo%2DCode)
    - 存储其中一个数组为 HashSet ，在对另一个数组进行 HashSet 查找

******************Rust Tips******************

- You can use `split_whitespace()` to avoid the `.filter(|n| !n.is_empty())`. Good job!
    - https://old.reddit.com/r/adventofcode/comments/18actmy/2023_day_4_solutions/kbx5j4v/

## Day 5

输入稍显麻烦，并不是特别的困难，对每行进行循环处理即可。问题中的 Map 其实就是对应关系，每一条 Map 由三个部分组成分别是 `dest` `src` 和 `length` ，可以将 Map 表示为函数，对于范围在 `[src, src+length)` 的输入数字 `input` ，可以通过如下计算得到转化后的 `result = dest - src + input` ，第一部分就是对输入的 `seed` 不断的进行计算最后得到 `location` ，选取其中最小值即可。

第二部分在第一部分的基础上进行扩展，每两个输入的 `seeds` 表示 `seed` 的区间，同样需要计算所有区间内 `seed` 经过转化最后得到的 `loaction` 中的最小值。首先尝试直接对区间内的每一个 `seed` 进行暴力求解，但是因为的实际输入范围较大，运行时间较慢，所以很快的放弃，在完成后阅读 Reddit 社区的解答后发现，实际上暴力求解虽然慢，但是并不是需要几天的那种，如果让它跑完，也许在十分钟内也能取得结果。

既然不选择使用暴力，实际上这个问题就是涉及区间的问题，如果能够对整个输入的 `seed` 区间，进行转换，那就减少了大量的计算，对一个输入 `input` 区间作用某一 `Map` 转换逻辑如下：

1. 将 `Map` 转为区间转换的表示方式：`[src, src + length)` ，同时 `offset = dest - src` ，那么就有 `[src, src + length) -> [src + offset, src + length + offset)`
2. 设输入区间 `input` 为 `[start, end)`
3. 那么只需要计算 `[start, end)` 和 `[src, src + length)` 的重叠区间 `overlaps` ，再加上 `offset` 即可得到输入区间 `input` 经过 `Map` 转换后的部分区间
4. 根据题意，如果一个 `seed`  无法被同一阶段的任意 `Map` 进行转换对应，那么就直接进行转换 （`10 → 10`）
5. 所以仅仅得到 `input` 和某一个 `Map` 的重叠区间仍旧不够，仍旧需要判断除重叠区间外的 `input` 区间是否能够被同一阶段的其他 `Map` 进行转换，也就是需要依次对剩余的 `input` 区间和剩余的同一阶段的 `Map` 进行转换，当所有同一阶段的 `Map` 都完成了转换后，如果 `input` 区间还剩下，那么依旧需要将剩下的区间保留到下一阶段，具体代码如下：
    
    ```rust
    fn convert_range(
        input: Range,
        dest: Number,
        src: Number,
        length: Number,
    ) -> (Vec<Range>, Option<Range>) {
        // src range: src..src+length
        // src and dest offset is: dest - src
        // then: dest = src + offset
        let offset = dest - src;
    
        let src_end = src + length;
        let (start, end) = input;
        if end <= src || src_end <= start {
            return (vec![input], None);
        } else {
            let overlaps = Some((start.max(src) + offset, end.min(src_end) + offset));
            // input range overlaps with range Number::MIN..src and range src_end..Number::MAX
            // is the remain range of input
            let remain_range: Vec<Range> = [(start, end.min(src)), (start.max(src_end), end)]
                .into_iter()
                .filter(|(a, b)| a < b)
                .collect();
            (remain_range, overlaps)
        }
    }
    
    fn convert_range_with_maps(range: Range, maps: &[SingleMap], converted: &mut Vec<Range>) {
        if maps.is_empty() {
            converted.push(range);
            return;
        }
        let (dest, src, length) = maps[0];
        let (r_ranges, overlaps) = convert_range(range, dest, src, length);
        if let Some(overlaps) = overlaps {
            converted.push(overlaps);
        }
        for r in r_ranges {
            convert_range_with_maps(r, &maps[1..], converted);
        }
    }
    ```
    
6. 根据输入，对每个阶段都进行同一阶段的 Maps 转化和对应，不断得到新的输入区间，直到完成所有的转换对应阶段，对最后得到的所有区间的起点取最小值即是第二部分的解

****************性能优化****************

- 采用 interval 区间重叠的方法可以视作是对暴力计算的一种性能优化
- 也可以在暴力计算的过程中改变每次循环的间隔进行逼近求解，当然这不是一种特别系统的方法
- 在利用 interval 求解的过程中依旧存在可能的性能优化，那就是对于每一个阶段输出的区间列表进行合并，减少下一个阶段的输入，涉及到区间合并的算法。
    - 首先按照区间起点对区间列表进行排序
    - 考虑两个**有序区间**合并的算法，即 `merge_range`，设区间：`[a_start, a_end)` 和 `[b_start, b_end)` 且 `a_start < b_start`
        - 如果 `a_end < b_start` 那么两个区间不存在重叠，区间 `a` 和 `b` 不需要合并
        - 反之，两个区间存在重叠，那么合并后的区间就是 `(a_start, a_end.max(b_end))`
    - 对于有序区间列表的合并只需要在这个基础上进行扩展即可，见 `merge_ranges`。
    - 首先对第一个和第二个区间进行合并，如果不存在重叠，那么第一个区间已完成了所有的合并，第二个区间再对第三个区间进行合并。
    - 反之如果存在重叠，那么合并后的区间，再对第三个区间进行合并。
    - 以此类推就可以对有序区间列表完成合并，具体代码如下：
    
    ```rust
    // this funtion only works when a.0 <= b.0
    fn merge_range(a: Range, b: Range) -> (Option<Range>, Range) {
        let (a_start, a_end) = a;
        let (b_start, b_end) = b;
        if a_end < b_start {
            (Some(a), b)
        } else {
            (None, (a_start, a_end.max(b_end)))
        }
    }
    
    fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
        if ranges.len() < 2 {
            return ranges;
        }
        ranges.sort();
        let mut merged_ranges = vec![];
        let mut next = 1;
        let mut remain = ranges[0];
    
        while next < ranges.len() {
            let (merged, remain_temp) = merge_range(remain, ranges[next]);
            if let Some(merged) = merged {
                merged_ranges.push(merged);
            }
            next += 1;
            remain = remain_temp;
        }
        merged_ranges.push(remain);
    
        merged_ranges
    }
    ```

## Day 6

输入的处理很简单，第一部分将两行输入转换为数组，第二部分则将两个数组分别组成一个大数。直接根据题意进行了暴力解答，题意理解如下：

- 对于给定的最大时间 `t` ，和给定最小距离 `d` ，求满足不等式 $(t - x)*x>d$ 的 `x` （按按钮的时间） 所有整数解

因为涉及的不等式并不复杂，而且第一第二部分给定的输入都并非超级巨大，所以暴力求解也可以轻松完成。但是既然涉及不等式，那么就可以简单的对不等式进行求解，取得答案，思路如下（虽然是初中数学，但是依旧进行了搜索，根本记不住公式，记住的部分也是错的，数学都还给老师了）：

- 对不等式进行转换 $-x^2 + tx - d > 0$
- ~~继续转换 $x^2 - tx + d <= 0$~~
    - 不等式两侧乘负数变号规则错误
- 正确的方程为 $x^2 - tx + d < 0$
- 可以发现，实际上要求的就是二次函数 $f(x) = x^2 - tx + d$ 在 ~~x 轴和~~轴下方时，x 的整数范围
- 同时也可以发现该二次函数在 x = 0 和 x = t 出是对称的，那么也可以通过计算当 $f(x) = 0$ 时 x 距离 0 或 t 的距离进行求解
- 当然最简单的方法就是直接计算一元二次方程 $x^2 - tx + d = 0$ 的两个解，求解公式为 $x = (t \pm \sqrt{t^2 - 4d}) / 2$
- 因为要求的是两个解的距离，就有以下方程 $(t + \sqrt{t^2-4d})/2-(t-\sqrt{t^2-4d})/2$ 化简后可得距离为 $\sqrt{t^2 - 4d}$
- ~~同时需要包含两侧的解，所以最后结果需要加一~~
- 因为需要计算的区间内的所有整数的数量，而直接接计算距离将无法考虑到这一点
- 设两个解为 `x1` 和 `x2` 且 `x1 ≤ x2`
- 如果 `x1.ceil() == x1` 为真则需要从 `x1 + 1` ，反之则需要从 `x1.ceil()` （包含）开始
- 如果 `x2.floor() == x2` 为真则需要从 `x2 - 1`, 反之所有的整数直到 `x2.floor()`（包含）为止
- 具体代码如下：
    
    ```rust
    let start = if x1.ceil() == x1 {
        (x1 + 1.0) as usize
    } else {
        x1.ceil() as usize
    };
    let end = if x2.floor() == x2 {
        (x2 - 1.0) as usize
    } else {
        x2.floor() as usize
    };
    end - start + 1
    ```
    
- **也可以从 x1.floor() + 1.0 开始到 x2.ceil() - 1.0 为止，两侧都包含 `(x2.ceil() - 1.0 - x1.floor() - 1.0) as usize + 1`**

## Day 7

题目并不难，为了构造正确的数据类型而花费了大量的时间，思路如下：

- 每一行输入包含 `Hand` 和 `bid` 两个部分，`Hand` 由五个字符即五张手牌组成，`bid` 为数字
- 根据组成 `Hand` 中手牌重复情况，存在七种可能的手牌组类型 `HandKind`
- 七种 `HandKind` 存在大小顺序，同时每一张牌也存在大小关系
    - 第二个部分调整了部分牌的大小顺序
- 在确定手牌 `Hand` 的大小时，首先考虑 `HandKind` 大小，再考虑牌的大小
- 第一部分需要对所有输入的手牌进行排序，从小到大依次乘上当前手牌的位置，再累加即可
- 第二部分引入小丑规则，即 `J` 牌为小丑，可以替代任何的牌，已达成更好的 `HandKind` ，但是 J 牌变为最小的牌
- 因为涉及到大小和排序，所以最简单的方法就是实现 `HandKind` 的大小比较，再实现牌的大小比较，最后再实现 `Hand` 的大小比较
- `HandKind` 可以使用 `enum` 表示，比较大小时则可以直接转为对应的数字进行比较
- 牌的大小比较也类似，首先字符转为对应顺序的数字，再对数字进行比较即可
- 确定 `HandKind` 的思路如下：
    - 利用 `HashMap` 统计手牌中每个牌的数量
    - `HashMap.into_values().collect()` 取得数量的数组，对数组进行排序
    - `values.iter().fold(0, |sum, i| sum * 10 + *i)` 将排序后的数组，转换为数字
    - 因为题意中的牌组类型较少，每一个 `HandKind` 通过这样转换后都能得到对应的唯一数字，一对一的关系，所以可以利用这个数字确定 `HandKind`
- 然后就是利用 `sort()` 对 `Vec<Hand>` 进行排序，并根据题意计算结果即可
- 第二部分稍微复杂点，相比于修改第一部分处理好的 `Vec<Hand>` 不如重新对输入进行处理，将牌 `J` 的大小置为最小，而用 J 牌替换其他牌构成更好的 `HashMap` 逻辑如下：
    - 依旧是利用 `HashMap` 统计手牌中每个牌的数量
    - 通过 `HashMap` ，如果 `Hand` 中 `J` 的数量介于 1 到 4 张（均包含），那么需要将 J 牌的数量从 `HashMap` 中移除，同时将对应 `J` 牌数量加到剩余牌数量最大的牌之上，实现的时候我是很自然的这样操作，但是实际上这是题目恰好的安排，如果改变了 `HandKind` 的大小顺序那么这样也许不可行。假设 `Hand` 中包含 `J` 牌，分析如下：
        - `14` 对应 `Four` ，无论 `J` 是一张还是四张，最后都能构成更大的 `Five`
        - `23` 对应 `Full` ，无论 `J` 是两张还是三张，最后都能构成更大的 `Five`
        - `113` 对应 `Three` ，无论 `J` 是一张还是三张，最后都能构成更大的 `Five` ，而不是变成 `23` 的 `Full` 这不是最优解
        - `122` `1112` 和 `11111` 都是同样的
- 依旧可以在处理输入时就对引入小丑规则的 `Hand` 和 `HandKind` 进行处理
- 花了大量的时间在输入处理上，第一部分因为细节上有错误，找了半天，第二部分则是因为一个可恶的生命周期问题，花费了太多时间，最后只好用不那么优雅的方法实现
- 部分代码如下：
    
    ```rust
    fn new(mut count: HashMap<i8, usize>) -> Result<Self> {
        let j_count = count.remove(&-1).unwrap_or(0);
        let mut values: Vec<_> = count.into_values().collect();
        values.sort();
        let count_number = values.iter().fold(0, |sum, i| sum * 10 + *i) + j_count;
        Ok(match count_number {
            5 => HandKind::Five,
            14 => HandKind::Four,
            23 => HandKind::Full,
            113 => HandKind::Three,
            122 => HandKind::Two,
            1112 => HandKind::One,
            11111 => HandKind::High,
            _ => return err!("Wrong hand: {:?}", values),
        })
    }
    ```
## Day 8（TODO）

第一部分直接按照题意思路求解即可，第二部分有以下几种思路。

1. **暴力**
    - 取得以 A 结尾的所有起始节点，在取得所有以 B 结尾的所有结尾节点
    - 每一次都按照指令对所有当前节点进行遍历，直到当前的所有节点与所有结尾节点相同
    - 但是实际输入巨大，而导致几乎不可能在有限时间内取得结果
2. **提前计算**
    - 按照第一部分的方法计算出所有的一个启示节点到一个结尾节点需要几步
    - 假如一个起始节点只能到达一个唯一的结尾节点，同时一个结尾节点也只能由一个唯一的起始节点到达，那么第二部分所求的就是这些路径长度的最小公倍数 lcm
    - 题目并未明确的指出起点和结尾是一一对应的，所以理论上是可能存在多种方式达成题意要求，而最后的结果则是这多种方式的最小值

### 环检测算法

- https://en.wikipedia.org/wiki/Cycle_detection#Floyd's_tortoise_and_hare

### DFS

### 性能优化

- 在环检测时，我直接使用了 HashMap 判断当前节点和指令是否出现过，可以采用环检测算法直接确定环的大小和起点
    - 环检测算法实际比 HashMap 的方法要进行更多遍历
    - 环检测算法节省了 HashMap 所需要的内存空间
- 以 HashMap 存储节点的左右节点，这影响了后续对节点路径的搜索的性能。用 Vec 替代 HashMap 存储 NetWork 最后 Debug 的运行时间从 1xx ms → 20ms


## Day 9

题目很简单，输入并不复杂，按行解析为数组即可，第一部分直接按照题目要求进行解答即可。第一部分思路如下：

- 迭代计算数组相邻两个元素差，将结果存入新的数组，直到新的数组全为 0
    - 计算过程中保留每一个数组的最后一个元素到 `last` 数组
- 当数组全为 `0` 时，反向计算上一层数组的最后一个元素，直到得到输入数组的最后一个元素
    - 设所有每一层数组的预测最后一个元素的数组为 predict 那么有:
        - `predict[0] - last[0] = predict[1]`
        - `predict[1] - last[1] = predict[2]`
        - 直到 `predict[n] - last[n] = predict[n + 1]`，且 `predict[n + 1] = 0`
        - 将所有等式两边相加得到： `(predict[0] + predict[1] + … + predict[n]) -  (last[0] + last[1] + … + last[n]) =  (predict[1] + predict[2] + … + predict[n+1])`
        - 化简可得 `predict[0] = Sum{ last } + predict[n + 1]`
        - 所以 `predict[0] = Sum{ last }`
    - **可以直接对 `last` 数组进行求和**

第二部分要预测输入数组的第一个元素：

- 迭代计算数组相邻两个元素差，将结果存入新的数组，直到新的数组全为 0
    - 计算过程中保留每一个数组的第一个元素到 `first` 数组
- 当数组全为 `0` 时，反向计算上一层的数组的第一个元素，直到得到输入数组的第一个元素
    - 设所有每一层数组的预测第一个一个元素的数组为 predict 那么有:
        - `first[0] - predict[0] = predict[1]`
        - `first[1] - predict[1] = predict[2]`
        - 直到 `first[n] - predict[n] = predict[n + 1]`，且 `predict[n + 1] = 0`
        - 为了避免计算每一个 predict 值，可以每隔一个等式改变等式两边的符号，再对所有等式进行累加
        - `(first[0] - predict[0]) + (first[1] - predict[1]) = predict[1] - predict[2]`
        - 化简可得 `first[0] - first[1] = predict[0] - predict[1]`
        - 依次类推可得 `first[0] - first[1] + first[2] -first[3] + … + fisrt[n-1] - first[n] = predict[0] - predict[n + 1]`

**实际上第二部分存在一种取巧的方法，可以将输入的每一个数组都进行倒序，然后按照第一部分方法计算即可。**

## Day 10

### ****************************************Brute Force and BFS [[code](https://github.com/livexia/advent-of-code-2023/blob/51559bfee7d0b08eccb3ca62c77e078c6ca4d583/aoc10/src/main.rs)]****************************************

- 并没有按照方向前进
- 而是遍历当前 `Pipe` 四周所有的 `Pipe`
- 根据两个 `Pipe` 的链接情况确定下一个 `Pipe` 的位置
- 递归遍历**没有访问（搜索）过的位置**，直到无法确定下一个 `Pipe`
- 因为不确定起始点的 `Pipe` 类型，起始点的 `Pipe` 可与邻接的四个 `Pipe` 都连通
- 使用 `BFS` ，确定 `BFS` 最长路径长度，第一部分的结果是最长路径长度的一半

### **************************Move along and DFS/BFS [**************************[DFS](https://github.com/livexia/advent-of-code-2023/blob/74afe4cc89c7fcb143d1c9da6da1c93e53eb06ab/aoc10/src/main.rs)**************************/BFS]**************************

- 根据水管的方向进行移动
- 初始时同样存在四种可能的移动方向
- 根据移动方向计算下一个位置坐标
- 取得下一个位置坐标处的水管类型，确定移动方向是否能与该水管连通
- 如果无法连通，则当前最初的移动方向存在错误
- 如果可以连通，则更新当前坐标，同时根据水管类型更新移动方向
    - 第一部分可以只计算路径长度，也可以保留所有连通水管的坐标，第二部分实际上是需要所有连通水管的坐标的
- **直到回到原点**，表明初始的移动方向正确，结合最后的移动方向，可以确定初始水管的类型
- 可以用 BFS 或者 DFS 实现，不过 DFS 可能会栈溢出

第一部分上述两种思路和方法都可以得出正确的解，但是对于第二部分，实际上第二种方法更加合理，从思路和逻辑上看也是第二种方法更加清晰，更容易理解题意。尝试在第一种方法的基础上实现第二部分，耗费了我大量的时间，同时因为这种方法存在混乱，使得迟迟无法对第二部分作出有效的解答，所以狠下心来重新按照第二种方法解决了第一部分，也较容易得处了第二部分所需要的循环路径上的节点，当然第二部分也依旧没有因此而快速解决，**以下几种解决第二部分的方法，我只实现了第一种 Double Resolution / Flood Fill ，其余的方法都来自 Reddit ，我认为都是要优于我自己想到的方法。**

### **Double Resolution** / **Flood Fill [[code](https://github.com/livexia/advent-of-code-2023/blob/6a43dbb06d8346383f1d85f887dd4ba42221927e/aoc10/src/main.rs#L187-L253)]**

- 利用上述第一部分的第二种方法，计算循环的路径保存为 `loop_path`
- 两倍放大网格，在原由的每一个`Tile` 四周填入新的 `Tile`
- 构造二维数组表示网格，为 `expand_map` ，元素为 `1` 表示处于循环，元素为 `0` 表示被循环包含，元素为 `2` 则表示不被循环包含
    - 也可以使用 `false` 表示不被包含，`true` 表示被包含，但是在判断过程中，要将补齐的 `Pipe` 间的节点加入到所有的循环节点中，再确定当前的搜索节点是是否处于循环中
- 因为网格被放大了，在初始化 `expand_map` 时，要注意 `Pipe` 的坐标需要两倍放大
    - `expand_map[x1 as usize * 2][y1 as usize * 2] = 1`
- 同时需要补齐 Pipe 间的连接
    - 依次比较循环路径 `loop_path` 中的前后水管
    - 如果水管处于同一行，那么需要补齐左右两个水管间的节点
    - 如果水管处于同一列，那么需要补齐上下两个水管间的节点
- 同时要对 `expand_map` 的四边进行初始化，四个边上所有不是循环路径（补齐后）的元素都应当为 `2` ，即未被循环包含
- 完成了 `expand_map` 的初始化后，可以从四边未被循环包含的节点开始搜索，当前搜索节点的四个邻接节点中为 `0` 的节点置为 `2` 即可，利用 BFS 或者 DFS 完全全部搜索
- 最后将 `expand_map` 缩小，去除所有奇数行和奇数列的节点，统计所有为 `0` 的节点数量即是第二部分的解

### Line Crossing (**Ray Casting) / Shoelace Theorem? [[code](https://github.com/livexia/advent-of-code-2023/blob/9bb4e26da49eece00fccc432614d83b886cac76e/aoc10/src/main.rs#L287-L322)]**

这个方法我根本没想到，记忆里往年也是有用这个方法的，这个方法不算是平时算法练习里会出现的，感觉上这个方法是正确的，但是并没有阅读过完整的证明，即我********************************只知其然，不知其所以然********************************参，考链接如下：

- https://old.reddit.com/r/adventofcode/comments/18evyu9/2023_day_10_solutions/kcr28lt/
- https://en.wikipedia.org/wiki/Ray_casting
- https://gamedev.stackexchange.com/questions/141460/how-can-i-fill-the-interior-of-a-closed-loop-on-a-tile-map

**算法概述**

- 想象从闭合曲线外的一个点发射一条直线与曲线相交
- 当直线与曲线未相交时，直线的所有部分都在闭合曲线外
- 当直线与曲线相交一次，发射点到交点部分都在闭合曲线外，而交点到当前直线末端的部分在闭合曲线内
- 当直线与曲线相交两次时，第一个交点到第二个交点部分都在闭合曲线内，而第二个交点到当前直线末端的部分在闭合曲线外
- 以此类推，可以发现对于所有的**奇数 N ，第 N 个交点到第 N + 1 个交点间的直线线段都在曲线内**
- 也可以换一种思路
- 想象从闭合曲线外的一个点发射一条直线与曲线相交，初始闭合曲线入度为 0，出度为 0
- 当直线与曲线第一次相交时，闭合曲线的入度加 1 ，为 1
- 当直线与曲线第二次相交时，闭合曲线的出度加 1 ，为 1
- 即每当奇数次相交时，入度加 1 ，而偶数次相交时，出度加 1
- 当入度和出度相同的时候，直线的这部分不被曲线包含
- 当入度比出度大一的时候，直线的这部分被曲线包含

**具体思路**

- 依旧利用上述第一部分的第二种方法，计算循环的路径保存为 `loop_path` ，结果保存在 `result` 中
- 按行遍历网格，即从当前行的最左侧向右发射直线，默认相交计数为 0 ，即 `crossing_count = 0`
- 依次按行向右判断，如果当前节点为 `Pipe` ，说明当前行与循环相交或者相切
- 如果当前 `Pipe` 为 `-` 那么就是相切，不必考虑
- 如果当前 `Pipe` 为 `|` 那么必然相交，相交计数加一 `crossing_count += 1`
- 当 `Pipe` 为弯角即 `F J L 7` 的一种时，需要考虑上一次遇见的弯角水管 `last_corner`
    - 如果`last_corner == F && pipe == J` ，是竖着的折现，那么是相交的情况 `crossing_count += 1`
    - 如果`last_corner == F && pipe == 7` ，是 U 型管，可以想象当前行实际上是跟着水管走了，那么是相切的情况，不必考虑
    - 如果`last_corner == L && pipe == 7` ，是竖着的折现，那么是相交的情况 `crossing_count += 1`
    - 如果`last_corner == L && pipe == J` ，是倒的 U 型管，可以想象当前行实际上是跟着水管走了，那么是相切的情况，不必考虑
    - 对四种情况进行分析，可以发现只需要在当前水管是 7 或 J 的时候对相交计数加一 `crossing_count += 1` 即可，无需记录上次遇见的弯角水管

### ****************************More Solutions****************************

**计算机图像处理?**

- https://old.reddit.com/r/adventofcode/comments/18evyu9/2023_day_10_solutions/kcqufnn/
    
    > [Part 2](https://github.com/PaigePalisade/AdventOfCode2023/blob/main/Solutions/day10part2.c) was a bit of a struggle for me. At some point I figured that the best way of solving it was to print the path with box building unicode characters, put a screenshot of it into gimp, fill the inside of the path with the fill tool, and then count the number of periods in the filled region. For some reason, this didn't work and I'm not going to debug that janky solution.
    > 

**标记循环移动方向同一侧邻接节点（未实现）**

- 从循环起点顺时针或者逆时针前进时，前进方向同一侧（右侧或左侧）的所有节点要么被循环包含，要么不被循环包含
- 标记前进方向同一侧（右侧或左侧）的节点
- 对这些标记节点进行 BFS 或 DFS 对所有邻接且不在循环上的节点进行标记，直到完成所有节点的遍历
- 判断左上角第一个节点是否被标记
    - 如果被标记则意味着该节点不在循环上，同时该节点接墙，那么所有标记的节点都不被循环包含，所有节点数减去循环节点数再减去标记节点数，即是被循环包含的节点
    - 如果不被标记，那么该节点原来应当是 Pipe ，同时也处于循环，那么所有标记的节点都应当被循环包含，直接统计标记节点数即可
- https://old.reddit.com/r/adventofcode/comments/18evyu9/2023_day_10_solutions/kcqnr5i/
    - https://github.com/yangdanny97/advent-of-code-2023-rust/blob/main/src/day10/mod.rs

### Day 11 [[code](https://github.com/livexia/advent-of-code-2023/blob/main/aoc11/src/main.rs)]

今天的题目思路简单明了，输入为网格，记录其中字符为 `#` 的位置，第一部分要求将网格的中不包含任何的行和列都翻倍，而第二部分则是将这个倍数增加到 `1000000` 倍，要求计算翻倍后每一对唯一的 `#` 间的最短距离。

******************网格翻倍：******************

- 第一第二部分仅有扩张倍率不同，设变量 `expansion_rate` 表示
- 可以先考虑对行进行翻倍，再以同样的方式考虑列
- 初始 `empty_row = 0` 即当前未遇到任何不包含 `#` 的行
- 遍历每一行
- 对当前行上所有的 `#` 的坐标进行扩大， `(x, y) → (x * empty_row * (expansion_rate - 1))`
    - 要注意要对 `expansion_rate - 1` ，因为是变为 `expansion_rate` 倍，而不是增加
- 如果当前行上没有任何的 `#` 则对空行计数加一， `empty_row += 1`

**最短距离**：因为是网格，同时网格间并不存在任何阻碍，所以直接计算曼哈顿距离即可 $D=∣x1​−x2​∣+∣y1​−y2​∣$

****************性能优化****************：利用 `HashSet` 保存 `#` 的坐标，固然可以在查找时避免麻烦，但是在这个问题中完全没有必要，可以直接用 `Vec` 保存。

## Day 12 (TODO)

### **DFS with Cache [[code](https://github.com/livexia/advent-of-code-2023/blob/cb7133436a0e11be14d1e5a299024265af503ba6/aoc12/src/main.rs)] =** DP ?

- https://old.reddit.com/r/adventofcode/comments/18ge41g/2023_day_12_solutions/kd0um89/

 if springs[i] == ‘.’ dp[i] = dp[i - 1]

if springs[i] ≠ ‘.’ count += 1

### Rust @

https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#-bindings

## Day 13

题意要求查找矩阵的对称轴，题目并不难，逻辑很简单，因为对称轴可能是出现在行上也可能是出现在列上，可以先计算一个方向上的对称轴，再将矩阵转置在重复计算即可，直接暴力实现。

虽然题目的输入处理很简单，同时逻辑也不复杂，输入的数据量也不大，所以理论上应该是很容易的题目，但是我在第二部分卡了很长时间，原因就是在第二部分题意的理解上，第二部分有一句题目描述为 “you discover that every mirror has **exactly** one smudge” ，我没能注意到其中的关键词 **exactly** 以致于浪费了太多的时间，具体思路[[code](https://github.com/livexia/advent-of-code-2023/blob/e512830206b964989fcafc9127effd2746b436c9/aoc13/src/main.rs)]如下：

- 以查找垂直方向（列）的对称轴为例，遍历所有可能的对称轴位置 `i`
- 因为对称轴一定位于两个元素之间，而不在矩阵上，则令
- `left = i - 1` 和 `right = i` 同时不同计数器 `diff_cnt=0`
- 对于每一对 `left` 和 `right` 遍历每一行，如果当前行上 left 和 right 位置处元素不同
- 对计数器 `diff_cnt` 加一
    - 第一部分：此时表示当前对位置 `i` 一定不可能时对称轴，可以直接检测 `i + 1`
- 直到 `left` 或者 `right` 到达边界
- 第一部分要求查找对称轴左侧（上方）的列（行）的数量，结果即是对于对称轴 `i` ，最终 `diff_cnt` 为 `0`
- 第二部分要求在矩阵中允许**有且仅有一对元素不对称**，计算此时的对称轴 `i` ，思路和第一部分一致，结果即是对于对称轴 `i` ，最终 `diff_cnt` 为 `1`

### 性能优化

**利用数字表示矩阵的每一行和每一列，同时用位运算进对称比较 [[code](https://github.com/livexia/advent-of-code-2023/blob/8857d18cedb88ae689b00f48bee082797e86a8f5/aoc13/src/main.rs)]**

- 输入的矩阵只包含两种不同的字符，同时输入矩阵的长和宽的大小均在数字可表示的范围内
- 二进制数字每一位表示该行（列）在该位置上的字符
- 对比两行本来需要遍历两行，进行元素的一一比对，统计不同的数量，而利用数字表示之后
- 只需要对两个数字进行异或运算，结果的二进制表示中 1 的数量即是不同的数量

## Day 14 [[code](https://github.com/livexia/advent-of-code-2023/blob/main/aoc14/src/main.rs)]

第二部分涉及环检测，使用了第八天里也用到的环检测算法，计算环的起始点和大小，再取得最后结果，题目并不难，但最后我对性能并不是特别的满意，所以在具体平台倾斜的实现上进行了优化。

### 环检测算法 [[code](https://github.com/livexia/advent-of-code-2023/blob/aff91ffe7fa0ec97538af098d6a791babe078f95/aoc14/src/main.rs#L111-L134)]

往年我都是要用哈希表进行环检测，今年遇到环的问题，我都选择用快慢指针算法实现，相比于哈希表，两种方法的对比：

- 哈希表进行环检测时，要确保键值的设置合理，在这个问题里如果只将计算出的 load 值作为键值，会错误的检测到环，而如果用平台当前的状态（Vec）作为键值则又要考虑可能的内存消耗。如果循环较小这个问题还不严重，但是如果循环较大，那么利用哈希表也许就不可行。
- 用 Floyd 的快慢指针算法，虽然在检测的过程中，需要使用两个指针进行移动，也许检测中所需的指令较多，但是检测过程中最多只需要保存两个状态（如果算上初始状态，那么就是三个状态），内存消耗较少。
- 哈希表环检测使用空间换时间，而快慢指针算法则是用时间换空间

************************************Floyd 快慢指针************************************

- 虽然是快慢指针，但是这个算法适用于所有的环检测，而不仅限于链表，指针指代的是状态转换，以下用乌龟表示慢指针，兔子表示快指针
- 算法概述：龟兔赛跑，分成三个部分：检测环，确定环入口，确定环大小
- **检测环**
    - 乌龟，以正常的速度跳转到下一个状态
    - 兔子，以两倍的速度跳转
    - 如果过程中两个状态相同，那么说明存在环
- **确定环入口**
    - 此时将乌龟放回起点，兔子和乌龟都以正常的速度跳转到下一个状态
    - 当乌龟和兔子再次相遇，此时兔子的位置就是环的入口
- **确定环大小**
    - 兔子不动，乌龟以正常速度跳转
    - 再次相遇时，兔子跳转的距离即是环的大小
- 这个算法只属于能用的情况，具体的证明还需要进一步学习
- https://en.wikipedia.org/wiki/Cycle_detection#Floyd's_tortoise_and_hare

### 性能优化

- 当前倾斜平台的实现是按照模拟的方式，矩阵平台倾斜的情况
- **代码重构：**第二部分需要向四个方向倾斜，最初我直接复制黏贴实现了四个函数，可以将四个函数进行合并，合并之后性能有所下降 **700ms → 1s** 。[[code](https://github.com/livexia/advent-of-code-2023/commit/f13789ce924e2d3452d4c0baac92fa6470885d23)]
- 倾斜过程中圆石没有一次移动到位，而是一个位置一个位置的移动，可以直接一步到位，避免不必要的修改，性能提升不明显 **1s → 900ms**
- 考虑一行石头滚到右侧的情况，当前的逻辑是从最右侧的圆石开始移动，计算新的位置，虽然不是一个位置一个位置的移动，但是依旧是一个位置一个位置的比较，应当有更优解：
    - 以行 `OOOO…#`  为例初始时位置为各个位置 0 1 2 3 4 5 6 7
    - 最右侧的第一个圆石位置为 3 ，右侧与它最近的石头位置为 7，移动后圆石新的位置为 6
    - 移动右侧第二个圆石时，理他最近的石头位置为 6 ，所以它新的位置为 5，依次类推
    - 代码如下
    
    ```rust
    fn tilt_north(platform: &mut [Vec<char>]) {
        for j in 0..platform[0].len() {
            let mut last_possible = 0;
            for i in 0..platform.len() {
                if platform[i][j] == '#' {
                    last_possible = i + 1;
                } else if platform[i][j] == 'O' {
                    platform[i][j] = '.';
                    platform[last_possible][j] = 'O';
                    last_possible += 1;
                }
            }
        }
    }
    ```
    
- 在四个方向上应用新的移动算法，虽然需要在四个方向上分别存在以一个函数，但是性能提升明显 **900ms → 500ms** 。[[code](https://github.com/livexia/advent-of-code-2023/commit/2279c94738b3e9134d33da64d77f842a5c9bd551)]
- 也许可以使用数字代表平台的每一行？
    - 矩阵每一个位置有3种情况：圆石 `O`、方石 `#` 和空地 `.`
    - 如果 1 表示当前位为石头，0 表示空地，那么就需要额外维护一个列表记录所有圆石的位置
        - 某种程度上而言，这又是二维？
- 也许可以优化环检测中的状态对比？当前是直接比较两个 Vec ，好在 Vec 并不大

## Day 15

第一部分需要实现哈希，第二部分则需要实现类似于哈希表。第一部分按照题意实现即可，第二部分的题意有点复杂，但是理解题意之后其实也是很简单的。输入的数据量也不大，直接实现即可。第一部分不需要说明，简单说明第二部分：

- 输入字符串由逗号隔开分成一个一个部分
- 每一个部分由两到三个部分组成，初始连续的字符为镜头的标识符 `label`
- 紧接着是 `=` 或 `-`
    - `=` 对应增加或更新操作
    - `-` 对应删操作
- focal length 在最后的结果计算中需要
- 将每个部分的字符串转为 `ascii` 码，然后进行分割，分割实现如下：
    - 直接根据是否能够从中解析出 focal length 来确定当前操作是 `=` 还是 `-`
    
    ```rust
    fn step_to_instr(step: &[u8]) -> (usize, usize, Option<usize>) {
        let i = step.iter().position(|&c| c == b'=' || c == b'-').unwrap();
        let k = hash(&step[..i]);
        let v = step.get(i + 1).map(|n| (n - b'0') as usize);
        assert!(i <= 8); // usize is u64, max lable length is 8
        let label = step[..i].iter().fold(0, |l, &b| l * 256 + b as usize);
        (k, label, v)
    }
    ```
    
- `label` 字符串的哈希值表示当前镜头需要放置哪一个 `box`
- `=` 之后还有一个数字，表示镜头的 focal length
    - `=` 代表需要将对应镜头和对应的 focal length值放到对应的 `box` 中
        - 如果对应镜已经存在，那么修改原有 focal length 为新的值
        - 如果不存在则将当前镜头放置到最末端
- `-` 之后则什么都没有
    - `-` 表示需要从对应的 `box` 中移除对应镜头
- 用二维 `Vec<Vec<(label, focal legth)>>` 表示所有的 `box` 和其中的镜头
- 我的实现中 `label` 的数据类型是 `usize` ，避免了构造 String 但是同样的镜头标识长度不能超过 8 。`let label = step[..i].iter().fold(0, |l, &b| l * 256 + b as usize);`
- 利用 position 确定当前镜头标识是否已经在 box 中，如果在取得索引值，再根距对应的 `=` 和 `-` 进行更新

部分处理代码如下：

```rust
let mut map = vec![vec![]; 256];

for step in steps {
    let (k, l, v) = step_to_instr(step);
    let p = map[k].iter().position(|(i, _)| i == &l);
    match (v, p) {
        (Some(f), None) => map[k].push((l, f)),
        (Some(f), Some(i)) => {
            map[k][i] = (l, f);
        }
        (None, Some(i)) => {
            map[k].remove(i);
        }
        _ => (),
    }
}
```

## Day 16

今天的问题并不难，光线按照方向移动，根据新位置处可能的镜子调整移动方向，计算光线移动的路径，除了方向的改变，光线在遇到特定的镜子时还会产生分裂，我通过利用 BFS 实现了第一部分。第二部分是在第一部分之上计算从四周发出光线，每个位置发出光线移动路径的最大值。唯一需要注意的点就是光线的起始点，如果光线在起始点就遇到了镜子，那就需要根据镜子先调整方向。这点可以通过在平台外设置一个虚拟的起点解决，也可以首先对起点和起点处的镜子计算下一个可能的方向。代码的实现很简单，可是性能却并不好，第二部分主要的运行时间都是在 BFS 中对当前状态的访问情况进行检查上，而我在尝试提升性能时引入了错误的逻辑，导致性能提升，但是结果错误。

**错误**的调整：**~~调整了 BFS 中 visited 的检测位置，代码运行时间从 4s → 80ms [[code](https://github.com/livexia/advent-of-code-2023/commit/2d21d515193a26817ee6f237e4f89d474a65654f)]~~**

- **出队检测**：左侧代码先将下一个状态推入队列，等到再次从队列中取出时再检测状态是否在 `visited`
    - 因为是出队时检测，那么队列中就会存在多个重复的状态
    - https://stackoverflow.com/a/45643883
    - https://stackoverflow.com/a/25992077
    - https://stackoverflow.com/a/63322362
- **入队检测**：右侧则是确定**下一个状态**不在 `visited` 中后，再推入队列
    - 右侧代码保证队列中的每一个状态都是未被访问过的，队列中重复的元素更少，这控制了队列的大小，相应的也就减少了 `insert` 的操作次数
- 对左右两种实现统计 visited 的大小，可以发现左侧代码中 visited 大小可以比右侧 visited 大百倍 (11117 vs 57)，**如果仅仅是因为检测时机的问题不应该会导致 visited 大小有剧烈的变化**
- **右侧的代码理论上应该是错误，实际上右侧代码的 for 循环是没有必要的，它只将其中一个状态加入队列，实际上是减少了遍历的分支。~~理论上这是错误的，但是为何运行结果是对的？~~**
    - `visited.insert((pos, dir))` 应该是 `visited.insert((next, n_d))` 才对
- 理论上一个状态经过反弹最多存在两种状态，当光线是向左或向右遇到 `|` 时会产生两种新的状态，或者是当光线是向上或向下遇到 `-` 时也会产生两种新的状态。右侧错误的代码实际上之旅两种状态中的第一种，直接忽略了可能的第二种情况。

![image](https://github.com/livexia/advent-of-code-2023/assets/15051530/3ba76f7f-21b5-4bd1-9f3f-da247ea8584f)

### 性能提升

因为主要的时间损耗是在 BFS 中的访问检测上，那么就要考虑从这个方面入手。

- ~~第二部分中也许可以在所有起点的 `BFS` 中应用同一个 `visited` ，这个方法会在不同的 BFS 见产生干扰，导致结果错误~~
- 使用 with_capacity 创建 visited 和 energized 可以一定的提升效率 [[code](https://github.com/livexia/advent-of-code-2023/commit/5c6aec68747a91557032a244894bd67c32aec921)]
    - `let mut visited = HashSet::with_capacity(20_000);`
    - `let mut energized = HashSet::with_capacity(10_000);`
    - `cargo r` 4s → 2.8s release 500ms → 300ms
- 使用更好的 `HashSet` 实现或者 `Vec`
    - hashbrown 并没有明显的性能提升
    - Vec 需要使用三维数组才行

## Day 17 (TODO)

实现的思路并不复杂，但是求解空间巨大，无论用何种方法都需要剪枝，动态规划虽然不需要剪枝，但是因为路径选择存在前后依赖关系，需要保存的状态几乎等同于求解空间，也不现实。位置的移动并不是简单的上下左右移动一个，第一部分要求最长的同一方向移动距离为三，也就是说以一个方向移动三次必须要转向，那么对于当前位置最多存在六种可能的状态。这个地方存在两种处理，一就是只关心这六种可能的状态，省略中间状态，这个方法的好处是不需要考虑连续移动的次数，也一定程度减少了求解空间，~~但是却也影响了剪枝的范围~~（剪枝时依旧需要考虑当前连续移动的距离）。最后通过剪枝的 BFS 取得两个部分的解，但是应该可以用最短路径的算法进行求解。

### BFS

尝试通过 BFS 实现，但是在节点的访问状态判断上 BFS 并不正确，如果应用一个 visited 到整体的 BFS 会导致搜索路径的不完全，虽然节点是完全访问了，但是却无法取得所有的搜索路径，自然也就无法取得最终的结果，应该使用 DFS。

如果不使用 Visited 那么就要进行剪枝，引入 HashMap 记录当前位置和移动方向的最小 loss ，当遇到同样的状态时，判断当前 loss 是否小于记录值，如果不小于记录值，那么就不入队，实现剪枝。运行速度依旧堪忧，release 版本，运行需要 3s。第二部分和第一部分完全一致，引入节点的移动范围参数即可。

虽然没有对单条路径使用 Visited ，但是实际上如果在一条路径中，以同样的方向进入同一个节点两次，那么第二次的 loss 一定大于第一次，这样也就组织了可能的重复访问。

**如果能够证明在一条路径中，无论以何种方向进入同一个节点两次，第二次的 loss 一定大于第一次，那么就能进一步剪枝。**

### DFS

可能的路径太多，需要剪枝，~~引入缓存~~。应该可以用同 BFS 一样的方法进行剪枝。

**~~BFS + 最小堆~~：依旧需要剪枝，子问题的最优解之和并不等同于最优解。**

### 动态规划

前一次选择的路径会影响到下一次的路径选择，dp 应该是不能用的，(0, 0) 到 (1, 2) 的最短路径应该是 3 ，但是(0, 0) 到 (4, 6) 的最短路径中(0, 0) 到 (1, 2) 的路径长度应该是 5， 也就是说部分的最优解并不等同于最终最优解

```rust
1111999
9911999
9919999
9911199
9999111
```

### **Dijkstra's algorithm** (TODO)

- [https://en.wikipedia.org/wiki/Dijkstra's_algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm)
- 考虑输入 `abcde` 从 `a` 出发存在三条路径 `ab` `ac` `ad`
    - 如果选择了 `ad` 那么就不存在从 `d` 出发的路径

## Day 18 (TODO)

问题的输入处理并不复杂，第一部分根据输入首先构造边，再利用 Ray Cast 算法计算被边包含的大小即可。第二部分对输入的变化依旧不复杂，但是每一条边的长度都被放大，再用第一部分的方法，求解空间将会变得巨大，简单的将所有边包含的位置记录在 HashSet 中就需要及其长的时间，更别提在 Ray Cast 中需要遍历的一个一个位置了。

鞋带定理

https://en.wikipedia.org/wiki/Shoelace_formula

皮克定理

[https://en.wikipedia.org/wiki/Pick's_theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem)

## Day 19 [[code](https://github.com/livexia/advent-of-code-2023/blob/main/aoc19/src/main.rs)]

今天的题目依旧不算特别难，但是我在输入的处理上花费了太多的时间，第一部分的排名并不优秀，第一部分的实现根据题意实现模拟即可，注意细节即可。第二部分扩大了第一部分的求解空间，如果依旧按照第一部分的模拟实现，那么程序将无法完成运行，第二部分要求 `x m a s` 在 `[1, 4000]` 的范围内进行组合，计算所有组合中会被工作流接收的个数。如果构造组合，再根据第一部分实现的模拟一一判断，这是不现实的，所以需要另辟蹊径。问题太大的时候首先考虑对问题进行分解，初始问题描述如下：

- 对于 `x m a s` 在 `[1, 4000]` 的所有组合，从工作流 `in` 开始最后能被接收的组合个数

分解问题：以工作流 `in{s<1351:px,qqz}` 为例

- 经过 `in` 工作流的输入存在两种可能：
    - 如果 `s < 1351` 进入工作流 `px`
    - 否则进入工作流 `qqz`
- 那么原问题就变成，两个子问题：
    - 计算 `s` 在 `[1, 1350]` 和 `x m a`  在 `[1, 4000]` 的所有组合，从工作流 `px` 开始最后能被接收的组合个数
    - 同时计算 `s` 在 `[1351, 4000]` 和 `x m a`  在 `[1, 4000]` 的所有组合，从工作流 `qqz` 开始最后能被接收的组合个数
- 两个子问题的和即是原问题的解，同样的也可以对子问题再进行细分
- 可以**利用动态规划**实现，定义如下：
    - 函数 `A(wf)` 表示有多少 `x,m,a,s` 的组合经过工作流 `wf` 后能被接受
    - 转移方程： `A(wf) = Sum{A(next(wf))}` 其中 `next(wf)` 表示 `wf` 的后续工作流
    - 边界：当一个工作流 `wf` 的下一个状态为 `A` 时，所有经过 `wf` 的 `x,m,a,s` 的组合都被接受
- 定义 `possible` 表示经过工作流 `wf` 前所有的输入组合
- 在 `wf` 内，根据 `wf` 所有的可能下一状态进行对 `possible` 进行分裂
- 如果下一个可能状态为进入工作流，那么根据进入工作流的条件对 `possible` 进行分裂为 `occupy` 和 `remain`，`occupy` 为进入新的工作流的所有可能组合，递归求解子问题
- 剩余的可能组合 `remain` 成为 `possible`
- 如果下一个可能的状态为被接收，那么计算 `possible` 的组合数，输入的所有可能组合完成消耗
- 累加所有可能状态的被接收组合数

**[递归代码实现](https://github.com/livexia/advent-of-code-2023/blob/e8e56f4876c80256b6d91cf1a24efdaaaf4eb1f6/aoc19/src/main.rs#L220-L327)，也可以用队列迭代实现。**

## Day 20

第一部分根据题意模拟实现即可，求解空间并不大，循环一千次的模拟在可承受范围内。第二部分首先依旧尝试模拟实现，可惜短时间内根本无法取得解，最后得结果数量级是 100千亿 ，这不是模拟能够解决的，需要寻找更好的方法。自己虽然观察了输入，但是却没能想出具体的实现，最后参考了 Reddit 的题解，才终于得处答案。第一部分代码对任何的输入都可行，第二部分则只支持 rx 依赖一个 *Conjunction* 模块，同时这个 Conjunction 模块又依赖于其他的 Conjunction 模块，同时这些 Conjunction 模块发出高位是存在循环规律的，第二部分会给出每一个模块的可能循环大小，需要手动根据循环计算这些模块的最小公倍数，最后取得结果。

### 第二部分

观察给定的输入，可以发现 `rx` 仅依赖 `vd` ，而 `vd` 则依赖于 `rd bt fv` 和 `pr` ，`vd rd bt fv` 和 `pr` 都为 *Conjunction* 模块，所以如果 `rx` 要接收到低位，那么 `vd` 要发出低位，则 `vd` 的四个输入模块都必须发出高位，那么当 `rd bt fv` 和 `pr` 都发出高位时，`rx` 就能接收到低位。因为 rx 仅依赖于 `vd` ，那么求 `rx` 和求 `vd` 的求解空间应当是一样大的，所以就需要考虑 `vd` 依赖的四个模块，如果这四个模块存在一定的规律那么就可以将问题划分，减少求解空间。根据分析输入所得到的依赖关系，**利用代码计算这四个模块发出高位所需要的按键次数**，如果循环出现，那么说明存在规律，最后只需要计算四个模块发出高位次数的最小公倍数即是题解。

虽然第二部分涉及的代码是同第一部分一致的，但是在**利用代码计算这四个模块发出高位所需要的按键次数**的时间点很重要，因为 vd 这四个模块，所以要在经过 vd 模块就检测这四个模块发出的高低位，如果在按完一次按钮的最后，再检测这四个模块，那就可能无法检测到这四个模块的状态变化。这个细节也是为何我虽然知晓求解方法，但是依旧耗费了不少时间的原因。

## Day 21(TODO)

### 第二部分

- 可以从一个位置进行上下左右的移动，要计算从位置 `S` 开始，经过 `step` 次移动，可以到达位置的数，令函数 `f(step)` 表示该问题
- 同时令 `d(step)` 表示从从位置 `S` 开始，经过 `step` 次不回头的移动，可以到达位置的数
    - 即从 `S` 出发的最短路径长度为 `step` 的路径数
- 可以发现 `f(step) = d(step) + f(step -2)`
    - 当从 `S` 出发经过 `step - 2` 次移动，如果剩余的 `2` 次移动允许回头，那么两次移动可以被消除（向左，再向右）
    - 所以所有 `f(step -2)` 的终点都包含在 `f(step)` 中
        - 同样的 `f(step - 4)` 被 `f(step - 2)` 包含
    - 同时 `f(step - 1)` 中的任何终点都不可能被 f(step) 包含，因为其中的位置，只需要移动一次就可以成为 `f(step)` ，而移动一次却不可能实现原地不动
- 如果网格中不存在任何的石头，那么 d(step) 的终点将是以 S 为中心的一个菱形的边，边上的任意位置与 S 的最短距离是曼哈顿距离
- 根据 `f(step) = d(step) + f(step -2)`
- 对于任意的偶数 `step` 有 `f(step) = d(step) + d(step - 2) + … + d(0)`
- 对于任意的奇数 `step` 有 `f(step) = d(step) + d(step - 2) + … + d(1)`
- 第二部分的网格是无限网格，所以就算知道 `f(step)` 的计算公式也不能减少多少计算量
- 参考 Reddit 上的解法，通过求解二次函数的系数，确定二次函数，再计算最终结果即可。
    - https://old.reddit.com/r/adventofcode/comments/18nevo3/2023_day_21_solutions/
    - 实际的输入中，网格的四边都为空，同时起始点 `S` 位于的行和列都为空，那么从 `S` 到四边上位置的最短路径长度即是二者之间的曼哈顿距离
    - 同时 `S` 位于网格的正中央，那么 S 到四角的最短距离长度刚好为网格的边长 `width`（长宽相等）
    - 所以从一个网格的 `S` 出发，在上下左右四个方向移动网格边长的距离 `width`，就会抵达相邻网格的 `S`
    - 当 S 移动 `width` 时 覆盖了 1 个原始网格
    - 当 S 移动 `2 * width` 时 覆盖了 5 个原始网格
    - 当 S 移动 `3 * width` 时 覆盖了 13 个原始网格
    - 当 S 移动 `4 * width` 时 覆盖了 25 个原始网格
        - [https://www.wolframalpha.com/input?i=quadratic+fit+{{1%2C+1}%2C+{2%2C+5}%2C+{3%2C+13}%2C+{4%2C+25}}](https://www.wolframalpha.com/input?i=quadratic+fit+%7B%7B1%2C+1%7D%2C+%7B2%2C+5%7D%2C+%7B3%2C+13%7D%2C+%7B4%2C+25%7D%7D)
    - 网格的覆盖是随着移动的距离二次增长的
- 第二部分要求计算移动距离 `26501365` 的位置个数
- 输入网格边长为 `131`，恰好 `26501365 = 131 * 202300 + 65`
- 可以计算出 65 次， 65 + 131 次 和 65 + 131 * 2 次移动距离时到达的位置数
- 假设二次函数为 $f(x) = ax^2 + bx +c$ 且 $x = (step - 65) / 131$
- 那么得到 f(0) , f(1) 和 f(2) 的值就可以计算出系数 a b c
- 再计算 f(202300) 即是结果

## Day 22[[code](https://github.com/livexia/advent-of-code-2023/blob/main/aoc22/src/main.rs)]

今天的题目并不难，核心的问题就是砖块掉落，是否会和下方的砖块产生碰撞。第一部分第二部分都是在这个基础之上进行求解。我首先就想到砖块实际上就是两个线段，是否会产生碰撞，那么只需要计算线段是否存在交点即可，参考了几个 stackover flow 的解法，可惜最后的答案不对。最后用了笨办法，那就是记录砖块占据的所有网格位置，碰撞时统计各自占据的网格位置是否存在重叠即可，笨办法替换了线性代数之后可以顺利解决两个部分，虽然运行速度不快。对比两个检测碰撞的方法，发现在两个线段存在重叠时，线性代数的方法会得出错误的结果，可惜我的线性代数学不到家，只看数学我是根本不知道问题何在。

**线性代数检测线段相交[[code](https://github.com/livexia/advent-of-code-2023/commit/fdb5d7cccfb93ee2c75de1382dde44b74238d6d0),** [fixed code](https://github.com/livexia/advent-of-code-2023/commit/10a2bb9480697a3a5ed8e26e179a436945aefb1d)**]**

- https://stackoverflow.com/questions/5666222/3d-line-plane-intersection
- https://stackoverflow.com/questions/55220355/how-to-detect-whether-two-segments-in-3d-space-intersect
- https://stackoverflow.com/questions/2316490/the-algorithm-to-find-the-point-of-intersection-of-two-3d-line-segment
- 当两个线段位于同一条直线上时，实现代码计算出的 s 和 t 都为 Nan ，于是被认定线段一定不存在交点（重叠）
- 增加线段重叠检测之后，运行结果正确，运行时间是 HashSet 方法的一半

**性能优优化[[code](https://github.com/livexia/advent-of-code-2023/commit/93c6f76316c688720026cb35380ffd4af26eb848)]**

- 无论是用 Set 还是线性代数判断线段是否相交时，可以先判断两个砖块在 Z 轴方向上是否存在重叠，如果不存在重叠，那么一定不可能相交，通过这一个判断可以将 Debug 编译的第一部分运行时间从 10s 降低至 3s  ，第二部分则从 18 s 降低至 6s。
- 当前的 HashSet 是存储在 Struct 中，每一个砖块都有自己的 HashSet ，在测试掉落时，也需要构造新的 HashSet ，也许可以使用一份 HashSet 记录所有砖块已经占据的位置。
    - Debug: 第一部分 3s → 2s

**构造支撑图[[code](https://github.com/livexia/advent-of-code-2023/commit/f1fecd6014c902769779cc9084eafd7c38bb7829)]**

根据初始完全落下所有砖块，构造砖块与砖块之间的支撑关系

- 第一部分要计算移除某一砖块（节点），图不被断开
- 第二部分则要计算移除某一砖块（节点），当图断开时剩余图的长度
- 主要的时间耗费应该是在最初的落下砖块和构造支撑图，第一第二部分的具体计算只占用很小一部分的运行时间
- 对比利用线性代数或者 HashSet 在第一第二部分都模拟消除砖块再检测下落，Debug 下的运行时间从 2s → 1s ，release 则是从 400ms → 120ms

## Day 23[[code](https://github.com/livexia/advent-of-code-2023/blob/main/aoc23/src/main.rs)]

需要计算最长路径，第一部分限制了部分节点的移动可能，于是很容易的利用 DFS 求解即可。第二部分则无法这样简单的实现，因为涉及到大量的可能路径，求解空间巨大，于是用 DFS 首先可能会栈溢出，然后就是运行时间过久，无法得出结果。即然简单的 DFS 不行，那么是不是可以引入缓存呢？可惜还是不行，缓存只记录在某一个路径中的最长部分，但是这个最长并不适用于另一个路径，也就无法正确的得出结果。通过搜索我发现还存在一种方法就是利用拓扑排序确定最长路径，拓扑排序要求是有向无环图，实现之后依旧无法计算出最长路径，原因就是在网格中移动的路径的确是有向无环图，但是如果把所有的可能路径都考虑到，那么实际上题目给出的网格是有环图，所以拓扑排序依旧不可行。

不优化的 DFS 方法 release 编译运行了 1268s 得出正确的解

可行的缓存：缓存需要记录的状态，当前经过的所有路径，这样可以确保不同路径之间不会产生干扰，可是需要记录的状态太大，不太现实？

### 将网格转化为图，并对图进行合并（剪枝）[[code](https://github.com/livexia/advent-of-code-2023/commit/1e0a24fdc04d821dece0e6e0d6be809dbb8e2a44)]

- 根据网格构造图
- 只记录邻接节点构成的长度为 1 的边，遍历网格中每一个不为 `#` 的位置，判断当前位置的所有邻接位置是否能够构成边
- 对图进行剪枝当一个节点只存在两个边时，可以删除当前节点并合并两个边，直到图中不存在只有两个边的节点
    - 经过剪枝，图的长度从 9412 减少为 36
    - 那么经过剪枝，最多只有 36 * 4 的边，求解空间大大减少
- 再对图进行 DFS 遍历
    - release 运行时间从 1268s 减少到 4s
- 除了因为边的合并的性能提升，相比于直接网格的 DFS 在对于邻接节点确定上也有一定的性能提升，700s → 600s
- https://en.wikipedia.org/wiki/Edge_contraction

### 性能优化[[code](https://github.com/livexia/advent-of-code-2023/commit/94a763044a57a4120b230947095c30ff5efc1133)]

- visited 不使用 HashSet 而使用 Vec ，运行时间：4s → 1s
    - 直接 dfs 的方法 1200s → 700s

## Day 24

今天的题目不是编程题，更像是数学题，第一部分要求二元一次方程组，第二部分要求九元九次方程组。第一部分因为涉及到的方程组数量较少，所以我手动推导出求解过程，代码中再进行计算。第二部分我则代码打印出方程，寻找到在线求解方程组的网站进行求解，第二部分这样居然进了前一千。**今天的问题主要就是需要计算直线相交的，这和第 22 天计算线段相交的问题实际上是一模一样的。涉及到数学和线性代数，我并不是完全理解，但是没办法，也就是这样了。**

题意分析：

- 所有的石头有一个初始位置，同时有一个速度，按照一定的时间间隔根据速度改变位置
- 第一部分需要计算两两石头的运行路径相交的次数，只要计算路径的相交，而不是石头产生碰撞，即相交不需要石头同时抵达。
- 第二部分则要想象一个新的石头，这个石头和所有输入的石头同时以各自的速度前进，而且这个石头会和输入的所有石头碰撞，计算这个石头的初始坐标和。

求解思路

- 令石头的初始位置为 p0 速度为 v ，运行时间为 t ，那么就有石头在 t 时的位置 $p = p0 + v*t$
- 第一部分判断两两石头路径是否相交，要注意的是相交的位置可能并不是正好处于整数的位置，所以第一部分的计算最好**使用浮点数作为计算类型**
- 令石头 a 的路径函数为 $pa = pa0 + va * ta$
- 令石头 b 的路径函数为 $pb = pb0 + vb * tb$
- 当路径相交时，即 pa = pb，所以就有 $pa0 + va*ta = pb0 + vb * tb$
- 第一部分只考虑 x 和 y ，所以自然的就存在两条方程
    - $xa0 + vxa * ta = xb0 + vxb * tb$
    - $ya0 + vya * ta = yb0 + vyb * tb$
- 因为 xa0 vxa xb0 vxb ya0 vya yb0 vyb 都是输入已知，那么只有 ta 和 tb 未知，同时有两条方程组，那么就可以通过消元法分别求解 ta 和 tb
- 如果计算出的 ta 和 tb 小于 0 那么说明石头a 和 b 的路径在他们还没移动前就相交了，不合题意
- 如果 ta 和 tb 是无穷，说明计算中存在除 0 的情况，那么石头 a 和 b 的路径应该是平行了，也就不可能相交
    - 根据无穷的 ta 和 tb 计算相交的位置，可以发现位置是在无限远的位置相交，实际上也就是不相交
- 计算出 ta 和 tb 之后，就可以确定路径相交的位置，判断相交位置是否位于输入给定的范围内即可
- [[code](https://github.com/livexia/advent-of-code-2023/commit/d1c9163f827d6926ef21c7a06b73f5f96574f69d)]第二部分需要计算当一个石头与所有输入石头都存在某一时刻直接相交，计算这个石头的位置
- 令这个石头的位置为 p 同时速度为 v
- 第二部分不仅需要考虑 x 和 y 也要考虑 z
- 令输入石头的位置和速度为 pn 和 vn ，n为输入的石头数，那么就有以下方程
    - $p + v*t1 = p1 + v1 * t1$
    - $p + v*t2 = p2 + v2 * t2$
    - $p + v*t3 = p3 + v3 * t3$
    - …
    - $p + v*tn = pn + vn * tn$
- 暂时先将 p 和 v 都视为一个变量（不是自然数，而是向量/矩阵）
- 首先存在 n 条方程，每一条方程引入一个新的未知量 tn
- 那么总共的未知量个数就是 n + 2
- 一次方程的数量要和未知量的数量一致才能求解未知量，可是总共只有 n 条方程
    - **方程并不是一次的，而是二次方程，这会影响结果吗？**
        - 改变方程的位置可以得到 $p - pn = tn(vn - v)$
        - p 和 v 实际上都是矩阵，所以在两侧叉乘上 $v - vn$ 得到
        - $(p-pn) \times (v-vn) = tn(vn -v)\times(v - vn)$
        - 同时有平行的向量叉乘为0， $(vn -v)\times(v - vn) = 0$ 即有
        - $(p-pn) \times (v-vn) = 0$
        - p 和 v 是未知数，同时 $p \times v$ 是每一个方程中可得到的共同部分，于是可以被消去，所以这个方程组虽然看似是二次的，可是实际上最后依旧是线性的
        - 因为这个原因所以就算不用第三方的求解工具，依旧可以手动推导求解
    - https://old.reddit.com/r/adventofcode/comments/18pptor/2023_day_24_part_2java_is_there_a_trick_for_this/kepvp8j/
    - https://old.reddit.com/r/adventofcode/comments/18pnycy/2023_day_24_solutions/kepu26z/
- 不过因为需要考虑三个方向上的情况，那么实际上一条方程是等同于三个方向上的方程
- 所以在考虑三个方向上后，p 和 v 实际上是总共 6 个位置了
- 未知量个数就是 n + 6，方程个数就是 3 * n
    - 实际上时间 t 可以被消去，当然 v 的三个未知量也可以被消去，不过这就要自己实现消元法了，头疼放弃。
- 满足未知量个数等于方程个数的最小 n 实际上是 3
- 所以理论上只需要计算 9 个方程，也就是输入中的 3 个石头就能计算出解，剩余的石头/方程是没有用的
    - 如果计算出的石头和剩余石头相交情况不满足题意，那么实际上输入是存在问题的，也就是说这样的输入是无法计算出可能的石头满足题意
- 九元九次方程如果直接用消元法手动推导求解，估计眼睛会看不过来，我用 Rust 生产了 9 条方程，再偷懒直接找了网站计算
    - [https://quickmath.com/webMathematica3/quickmath/equations/solve/advanced.jsp#c=solve_solveequationsadvanced](https://quickmath.com/webMathematica3/quickmath/equations/solve/advanced.jsp#c=solve_solveequationsadvanced&v1=x%2Ba*t%253D262130794315133%2B57*t%250Ay%2Bb*t%253D305267994111063%2B-252*t%250Az%2Bc*t%253D163273807102793%2B150*t%250Ax%2Ba*q%253D290550702673836%2B-74*q%250Ay%2Bb*q%253D186986670515285%2B19*q%250Az%2Bc*q%253D231769402282435%2B-219*q%250Ax%2Ba*w%253D275698513286341%2B-59*w%250Ay%2Bb*w%253D162656001312879%2B-24*w%250Az%2Bc*w%253D183065006152383%2B-225*w&v2=x%250Ay%250Az%250Aa%250Ab%250Ac%250At%250Aq%250Aw)
- 得出正确答案后，我尝试从 bard 和 cahtgpt 处取得正确答案，可惜 bard 不够聪明，chatgpt 倒是给出了一份 python 的代码，本地运行也的确计算出了正确答案。
    
    ```python
    from sympy import symbols, solve
    
    # 定义符号变量
    x, y, z, vx, vy, vz, t0, t1, t2 = symbols('x y z vx vy vz t0 t1 t2')
    
    # 定义方程组
    equations = [
        x + vx * t0 - (262130794315133 + 57 * t0),
        y + vy * t0 - (305267994111063 - 252 * t0),
        z + vz * t0 - (163273807102793 + 150 * t0),
        x + vx * t1 - (290550702673836 - 74 * t1),
        y + vy * t1 - (186986670515285 + 19 * t1),
        z + vz * t1 - (231769402282435 - 219 * t1),
        x + vx * t2 - (275698513286341 - 59 * t2),
        y + vy * t2 - (162656001312879 - 24 * t2),
        z + vz * t2 - (183065006152383 - 225 * t2)
    ]
    
    # 解方程组
    solutions = solve(equations, (x, y, z, vx, vy, vz, t0, t1, t2))
    
    # 打印结果
    print(solutions)
    # 计算 x + y + z
    sum(solutions[0][:3])
    ```
    

### 使用 Solver 求解方程

**z3 [[code](https://github.com/livexia/advent-of-code-2023/commit/fd68fc663e721ce1f5f13599a481a09857fedb30)]**

- https://docs.rs/z3/latest/z3/
- https://github.com/Z3Prover/z3/wiki#background
- `brew install z3` 之后依旧无法正确的找到 z3.h 应该是路径问题，不想排查，换一个 Solver
- 官方的文档不是很简洁明了，所以参考了社区的实现，最后的运行速度没有想象中的快
    - https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/24.rs
- 第二部分的题目要求 Rock 和输入的石头只会在一定的时间间隔内和输入的石头产生碰撞，而输入石头的位置和速度都是整数，所以最后碰撞的位置也一定是整数，那么求解的时候只需要考虑整数即可

**good_lp**

- https://crates.io/crates/good_lp
- 方程式存在两个变量相乘的情况，无法求解，back to z3

## Day 25

以为今天的题目会很简单，结果是我不熟悉的题目。给定的输入构成一个无向有环图，需要求解断开哪三个边可以使得图产生分裂，计算分裂后图的节点数乘积。因为不知道解决这个问题有什么特定的算法，所以我决定直接直接暴力遍历所有的可能边的组合，很可惜暴力是跑不出什么结果的。

- 从所有的边中选择三个删除总共存在 254960703480 这么多的可能
- 检测 100000000 个可能需要 2131.196800375s
- 预估时间：62 天

### 对边进行排序，提升暴力搜索的效率 [[code](https://github.com/livexia/advent-of-code-2023/blob/main/aoc25/src/main.rs)]

- 已知图在删除特定的三条边之后，图会被分成两个新的图
- 可以想象这两个新的图是由这三条边相连的
- **左边图中的节点和右边图的节点相连，那么至少要从这三条边中经过一次**
- 如果不断开任何边，那么从一个节点出发，通过 BFS 和 DFS 可以对其他所有的节点进行一次搜索（不重复访问节点）
- 在搜索过程中记录所有边的出现次数，当只从一个节点出发时，所有边的出现次数只会是 1
- 那么可以对图中所有的节点都进行单独的 BFS/DFS 搜索，并在过程中记录所有边出现的次数
- 同时在记录边出现的次数时，要对边进行统一化，即 (a → b) 和 (b → a) 应当记录在同一个边下
- 假设左图有 L 条边，右图有 R 条边，同左图和右图通过 3 条边相连
- 左图内的节点访问其他左图内的节点理论上不需要经过中间的 3 条边和右图的边
- 而如果左图的节点需要访问右图的节点，那么必然会经过中间 3 条边的某一条
- 在 BFS / DFS 的过程中不允许重复访问，那么每一次搜索一条边最多只会经过一次，同时有的边则会被忽略不访问（因为节点已经访问过）
- 但是每一次搜索一定至少会经过两个图之间三条边其中一条，如果没有经过，说明左图和右图的节点没有产生连通的路径，这和题意不符
- 虽然知道这样是可行的，但是我却没法明确的说出为什么，哎一知半解。
- 根据边出现的频率对边进行排序，那么出现次数前三的边就应该是需要被删除的边
- 暴力时可以优先删除出现频率高的边，判断是否可以分割图，直到分割

### 利用 Graphviz 生成图的可视化 [[pic](https://github.com/livexia/advent-of-code-2023/tree/main/aoc25/graphviz)]

- 将 input.txt 修改为 input.dot
- 利用 graphviz 的 neato 或者 spdf 生成图的可视化
    - 可以将图按照集群进行展示
    - neato https://en.wikipedia.org/wiki/Spring_system
    - spdf https://en.wikipedia.org/wiki/Force-directed_graph_drawing
- 根据可视化确定需要删除的边
- 将边删除，利用代码计算分裂的图节点数的乘积
- 这个方法太取巧，不是一个适用的方法
- https://graphviz.org/doc/info/lang.html
- https://graphviz.org/
