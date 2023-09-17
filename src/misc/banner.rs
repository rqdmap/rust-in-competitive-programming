// Rust Template for Competitive Programming.
// REPO: https://github.com/rqdmap/rust-in-competitive-programming
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠠⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡖⣢⣤⠀⠀⠀⠀⠀⠀⢀⣠⠤⠒⢲⣿⠟⣢⠤⠬⣷⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠓⠥⣻⢔⡦⢄⡀⠀⢀⠴⣀⣲⢀⣿⣊⠉⠀⠀⠀⠀⠈⠑⢤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠈⠑⢪⣁⡳⡾⢫⣾⣯⣿⡿⣧⡠⢖⣶⣦⢴⣦⣄⡻⣕⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣶⠏⠉⠁⠤⡀⢀⣀⣤⡄⠉⠉⠉⠙⠛⠿⣜⣄⡀⠁⠈⠊⢆⠳⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡀⠀⢶⠀⠀⠀⠀⠀⠀⠀⢀⡤⢚⠽⠊⢁⣠⣴⣄⡤⠀⠐⠛⠋⠁⠀⠀⠤⣀⠀⠀⠀⠀⠉⠛⢭⣒⠦⢼⡆⠘⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠳⡄⠘⡆⠀⠀⠀⠀⢀⠔⡡⠚⠁⢀⠔⠛⠉⠁⠀⠀⠀⠀⠮⠷⡄⠀⠀⠀⠀⠙⢦⡀⠀⠀⠀⠀⠈⠻⣶⣮⣍⡓⠢⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠙⠃⠀⠀⠀⢀⠜⢡⢎⣀⠀⠈⠁⠀⠀⢠⡿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠈⢻⣇⢹⣿⡒⠮⢕⣠⣶⣤⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠑⠶⠀⠀⠀⠀⡰⠋⢀⠟⢩⠋⠀⠀⠀⠀⢠⡷⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⡄⠀⠀⠀⠀⢠⠀⠀⠀⠀⠀⠀⠙⢿⡿⡇⡆⠀⠛⠿⣿⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⢎⡤⠰⠯⠒⠃⠀⠀⢤⡠⠀⡌⣁⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⡄⠀⠀⠀⠀⠱⡀⠀⠀⠀⠀⠀⠈⣽⠃⢰⠀⠀⠀⠀⠉⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠟⡝⠀⠀⠀⠀⠀⢀⠀⠀⠀⡸⠁⣇⠞⠀⡐⢹⠀⢀⠀⠀⠀⠀⡖⢳⡵⠀⠀⠀⠀⠀⢱⡀⠀⠀⠀⠀⠀⠸⡇⠈⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡼⡁⣼⠃⠀⠀⠀⠀⢀⡎⠀⢀⣾⡇⢇⡏⠀⠀⠑⠎⠀⣾⢰⡀⠀⠀⠓⠞⠃⠀⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⠀⣿⠆⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣾⠏⢾⠇⠀⠀⠀⠀⠀⣜⡜⠀⣾⢱⠀⠘⣷⠀⠀⠀⠀⢸⢹⢸⢣⠀⠀⠀⠀⠀⢠⠀⠀⠀⠀⠀⠈⡆⠀⠀⠀⠀⠀⣸⣆⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⠃⡜⡌⠀⠀⠀⠀⢀⢰⣟⡇⡼⠁⢸⠀⡀⠹⣧⠀⠀⡀⡎⠈⣼⠘⡆⢰⠀⠀⠀⠘⡇⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠉⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⡇⢰⣽⠁⠀⠀⠀⠀⣼⡿⠋⣿⠁⠉⢸⡆⣇⠀⠀⠑⠂⡇⢷⠤⢽⣧⣘⣌⣆⢀⠀⠀⢇⠀⠀⠀⢸⡄⠀⡆⠀⠀⠀⠀⠀⣼⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢷⡎⣟⠀⠀⡆⢀⠀⡿⣇⣳⣿⣤⣑⣎⢣⡟⣆⠀⠀⡀⡿⣼⠀⠀⠙⣄⠙⣿⣾⢧⠀⢸⢸⠀⠀⡀⢧⣇⡟⡀⠀⠀⠀⠀⡿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣵⠿⡇⢠⡇⢸⢸⣷⣿⡿⠋⣿⡿⣿⣶⠻⡈⠢⠤⣷⣵⠘⠃⣥⣦⣽⣦⣤⣯⠻⣳⣼⣸⣄⠀⢟⣼⣼⣿⠃⠀⠀⠀⠀⠇⢹⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⢠⡄⠀⠀⠀⠀⠀⠀⠀⢰⠛⣦⠡⠟⣇⢸⣿⣷⠹⡇⠀⣿⠷⣿⣿⠀⠀⠀⠀⠈⠉⠁⠸⠛⠉⣽⣟⢻⣿⣾⣌⣻⢿⣿⣦⣘⣾⣿⠏⠀⠀⠀⠀⡄⢸⠀⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⣈⣇⣀⣠⠄⠀⠀⠀⠀⠸⠀⠘⡀⣿⣿⣄⣻⣿⣇⠑⣄⠙⣄⠼⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣻⡏⣿⣿⢻⣿⣿⢸⣟⠿⢛⡭⢬⢢⠀⠀⡇⠀⢣⢸⡄⠀⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠈⠁⠀⠀⢧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢳⣇⠘⢻⡗⣿⣻⠀⠀⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣓⣀⣨⠷⠃⠁⡼⢸⡞⢠⢋⡓⠼⢸⠀⠀⢧⠀⠸⡀⡇⠀⠀⢣⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⢸⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠆⠹⠦⠄⡗⢟⢿⠑⠂⠀⠀⠀⠀⠦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠀⠀⠀⠀⡇⡈⢀⡼⢂⡼⣠⠞⠀⠀⢸⡀⠀⢇⢻⠀⠀⠀⠳⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣷⠂⠸⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣇⣇⠙⢋⡥⠚⢳⠀⠀⠀⠀⣇⠀⠘⡾⡆⠀⠀⠀⠹⣄⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⡟⠀⠐⡽⣆⠀⠀⠀⠀⠀⢤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣾⡯⣿⣽⠁⠀⠀⡈⢆⠀⠀⠀⠹⡀⠀⠈⢳⠀⠀⠀⠀⠘⢷⡀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⢀⡠⠂⠀⠀⠀⠀⠀⠀⠀⠀⠸⢩⠂⠀⡇⠏⣷⢄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠖⢩⣿⣿⡇⢹⡇⠀⠀⠀⢱⡘⡆⠀⠀⠀⢣⠀⠀⠈⢧⠀⠀⠀⠀⠀⠻⣦⡀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⡎⠀⠀⣿⢸⡟⡄⠑⣤⡀⠀⠀⠀⠀⠀⣀⣠⣴⠞⠉⠀⠀⢸⡛⠉⢳⢸⢹⡀⠀⠀⠀⣧⠘⡄⠀⠀⠈⠇⠀⠀⠘⣧⡀⠀⠀⠀⠀⠈⠫⡢⡀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⠀⡇⡆⠀⢸⣞⡇⢳⠀⠘⡟⠦⠤⣶⣾⣿⡿⠟⠁⠀⠀⠀⠀⢸⣇⠀⢸⣾⡄⡇⠀⠀⠀⠸⣧⠘⣆⠀⠀⠀⠀⠀⠀⠙⣧⡀⠀⠀⠀⠀⠀⠈⠺⡢⡀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⢠⣧⠀⢸⠸⣾⠘⠂⠀⠸⠆⠀⣹⡙⠉⠀⠀⠀⠀⠀⠀⠀⠸⡙⠛⠊⣿⡿⡇⠀⠀⠀⠀⢿⣧⠈⢦⠀⠀⠀⠀⠀⠀⠙⢷⡄⠀⠀⠀⠀⠀⠀⠈⠻
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣇⠀⠸⡿⡄⠈⢇⠹⣧⠀⠀⡲⣖⣾⣿⡇⠀⠀⠀⠀⠀⠀⠀⣀⣤⣷⠾⠟⢸⡏⢰⡀⠀⠀⠀⠘⣿⣧⡈⢳⡄⠀⠀⠀⠀⠀⠈⢻⢆⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⡄⠀⠙⢵⣄⢨⣦⣌⣳⡾⣹⣿⠿⢻⠇⠀⠀⠀⠀⣠⣴⠾⠛⠉⢀⠀⠀⠈⣧⠀⢿⡄⠀⠀⠀⠸⡻⣿⢆⠙⢆⠀⠀⠀⠀⠀⠀⠣⡳⢄⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠔⠒⡻⢿⣿⣧⣍⣩⠟⢠⡟⢦⡀⠀⠀⠀⢀⣴⡿⠋⠁⠀⠀⠀⠰⣣⡀⠀⠘⣧⡈⢿⣆⠀⠀⠀⢳⢻⡆⠱⡄⠳⡄⠀⠀⠀⠀⠀⠀⠀⠈⡢⢄⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠞⠁⠀⡰⢁⣼⠟⠹⠤⢶⠋⣾⡾⠀⡞⠴⢦⣴⠟⠯⢀⣀⠀⠀⠀⢀⡠⠵⠒⠊⢉⣉⣹⡮⣿⣦⡀⠀⠀⢣⢻⡄⠘⢿⠺⣦⡀⠀⠀⠀⠀⠀⠀⠙⢦⡉⠲
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠏⠀⠀⢠⡣⡾⠃⠀⠀⢠⠃⢸⡿⠃⠀⢀⣴⠟⠉⠀⠀⠀⡤⠤⠒⢋⠅⠀⣀⡠⠄⣠⣾⣿⣿⣿⣎⠀⠀⠀⠀⢳⣳⡀⠀⠳⡌⢮⡳⣄⠀⠀⠀⠀⠀⠀⠹⡦
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⡾⠀⢰⢀⣿⠟⠀⠀⠀⠀⡎⠀⣿⠇⢀⣴⠟⠁⠀⠀⠀⠀⠀⠀⣠⡴⠕⠒⠉⠀⠀⣰⡟⠀⠉⠈⠉⠛⡿⣶⣤⠈⢦⠹⣧⠀⠀⠈⢎⢷⣌⠳⣄⠀⠀⠀⠀⠀⠈
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣿⣧⡀⢸⡾⠣⠆⠀⠀⠀⢸⠀⢀⣿⣴⡿⠁⠀⠀⠀⠀⠀⠀⠀⠈⠁⠀⠀⠀⠀⠀⢰⠟⠀⠀⢀⣠⠖⠊⠉⠁⠸⣧⡀⠳⣹⡆⠀⠀⠈⢆⢻⢮⠎⠳⡀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣻⠹⣿⡟⡉⡷⠋⠀⠀⠀⢠⠀⢀⡆⠀⢨⣿⠏⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⠀⢰⡇⢠⠇⠀⣰⠶⠋⠀⠀⠀⠀⠀⠀⢸⢽⠀⠙⢿⠀⠀⠀⠘⡄⢳⡀⠀⢫⡲⣄⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡸⢇⠈⠻⡟⠀⠀⠀⠀⠀⠀⢇⢸⠁⣴⡿⠁⠀⠀⠀⢀⠤⠒⠁⠀⠀⠀⠀⠀⠀⣤⠀⡇⣜⣼⡸⠁⠀⠀⠀⠀⠀⠀⠀⠀⢸⣬⣀⠀⠈⢧⠀⠀⠀⠹⣆⢳⡄⠀⠹⣦⠑⢄
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡧⠬⠆⡤⠒⠒⠒⠒⣤⡀⢀⡈⠌⠀⠙⠁⠀⣠⣴⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠰⡏⡄⢷⣿⣷⢡⡿⠁⠀⠀⠀⣀⡠⠤⠤⢾⣇⣼⣿⡀⠀⠀⠀⠀⠀⠛⡄⢻⢆⠀⠀⢣⡀
// ⠀⠀⠀⠀⠀⠀⠀⢀⡞⡉⠁⣀⣼⠀⠀⠀⠀⠀⣿⣹⣶⣵⣒⢒⣢⣤⠉⣓⣒⡤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠳⠐⢾⣿⣋⣠⠔⣒⣒⣉⡁⠀⢀⣀⣀⣴⣛⣿⠟⣹⠂⠀⠀⠀⠀⠀⢻⠀⢇⠱⣄⣀⣣
