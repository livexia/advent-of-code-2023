# Advent of Code 2023

- https://adventofcode.com/
- https://github.com/livexia/advent-of-code-2023

## Rust Hints

- https://github.com/dtolnay/anyhow This library provides `[anyhow::Error](https://docs.rs/anyhow/1.0/anyhow/struct.Error.html)`, a trait object based error type for easy idiomatic error handling in Rust applications.
- `flat_map(|n| n.parse())` 会忽略 `Err` 保留 `Ok` 中的结果。

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
