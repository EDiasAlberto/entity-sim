import pygame
import numpy as np
import state_processor as sp 


def render_terrain(game_state, colour_dict, terrain_width, terrain_height):
    material_array, height_array = game_state.get_map_data()

    if material_array.ndim == 1:
        material_array = material_array.reshape((terrain_height, terrain_width))
        height_array = height_array.reshape((terrain_height, terrain_width))

    # Normalize heights 0–1
    h_min, h_max = height_array.min(), height_array.max()
    heights_norm = (height_array - h_min) / (h_max - h_min + 1e-9)

    terrain_array = np.zeros((terrain_height, terrain_width, 3), dtype=np.uint8)

    for material_id, base_color in colour_dict.items():
        mask = (material_array == material_id)
        brightness = 0.55 + heights_norm[mask] * 0.45  
        colour = (np.array(base_color)[None, :] * brightness[:, None]).clip(0, 255)
        terrain_array[mask] = colour.astype(np.uint8)

    surf = pygame.Surface((terrain_width, terrain_height))
    pygame.surfarray.blit_array(surf, np.transpose(terrain_array, (1, 0, 2)))

    return surf

def render_entities(game_state, width, height, entity_color=(255, 255, 0)):
    BACKGROUND = (0,0,0)
    entities = gs.get_entity_locations() 
    surf = pygame.Surface((height, width))
    surf.fill(BACKGROUND)
    surf.set_colorkey(BACKGROUND)
    
    for entity_id, x, y, is_alive in entities:
        # Draw entity as a circle
        entity_size = gs.get_entity_size(entity_id)
        render_color = entity_color if is_alive else (255, 0, 255)
        pygame.draw.circle(surf, render_color, (int(x), int(y)), entity_size)

    return surf


# Initialize game state
gs = sp.generate_game_state((800, 800, 10), (100, 100, 500, 500), 5)

# Define material colors
colour_dict = {
    0: (255, 0, 0),   # Mud - Red
    1: (0, 255, 0),   # Grass - Green
    2: (0, 0, 255),   # Ice - Blue
}

# Initialize pygame
pygame.init()
(WIDTH, HEIGHT) = gs.get_terrain_map()
screen = pygame.display.set_mode((WIDTH, HEIGHT))
clock = pygame.time.Clock()

# Render terrain once
terrain_surface = render_terrain(gs, colour_dict, WIDTH, HEIGHT)

# Render entities on top of terrain
entity_surface = render_entities(gs, WIDTH, HEIGHT)

# Game loop
running = True
while running:
    clock.tick(60)
    
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
        elif event.type == pygame.KEYDOWN:
            if event.key == pygame.K_SPACE: 
                gs.advance_state()
                entity_surface = render_entities(gs, WIDTH, HEIGHT)
            elif event.key == pygame.K_r:
                if event.mod & pygame.KMOD_LSHIFT:
                    gs.reset_game_state(False)
                else:
                    gs.reset_game_state(True)
                terrain_surface = render_terrain(gs, colour_dict, WIDTH, HEIGHT)
                entity_surface = render_entities(gs, WIDTH, HEIGHT)
    screen.fill((0, 0, 0))
    screen.blit(terrain_surface, (0, 0))
    screen.blit(entity_surface, (0, 0))
    
    pygame.display.update()
            
pygame.quit()

