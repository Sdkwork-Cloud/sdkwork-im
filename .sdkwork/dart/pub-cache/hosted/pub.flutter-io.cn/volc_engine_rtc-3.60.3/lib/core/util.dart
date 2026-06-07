/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

int combinedEnum(List<int> list) {
  int res = 0;
  list.forEach((obj) {
    res |= obj;
  });
  return res;
}
