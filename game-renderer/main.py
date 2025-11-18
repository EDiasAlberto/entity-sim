import pygame
import numpy as np
import state_processor as sp 

gs = sp.generate_game_state((800, 800, 10), (200, 200, 400, 400))
terrain_map = gs['terrain_map']
terrain_map_width = terrain_map['width']
terrain_map_height = terrain_map['height']

colour_dict = {
    0: (255, 0, 0),
    1: (0, 255, 0),
    2: (0, 0, 255),
}


map = np.ndarray((terrain_map_width, terrain_map_height, 3))

for i in range(terrain_map_height):
    for j in range(terrain_map_width):
        curr_idx = (i * terrain_map_width) + j
        curr_cell = terrain_map['map'][curr_idx]
        map[i][j] = colour_dict[curr_cell['material']]

cellsize = 1 
WIDTH = map.shape[0] * cellsize
HEIGHT = map.shape[1] * cellsize

pygame.init()
screen = pygame.display.set_mode((WIDTH, HEIGHT))
clock = pygame.time.Clock()
surf = pygame.Surface((map.shape[0], map.shape[1]))
pygame.surfarray.blit_array(surf, map)
surf = pygame.transform.scale(surf, (WIDTH, HEIGHT))

# game loop
running = True
while running:
    clock.tick(60)
    
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
            
    screen.fill((0, 0, 0))           
    screen.blit(surf, (0, 0))
    
    pygame.display.update()
            
pygame.quit()
