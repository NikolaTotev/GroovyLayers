-- User demo1
INSERT INTO groovy_layers.users (email, username)
VALUES ('demo1@exmaple.com', 'demo1');

INSERT INTO groovy_layers.material_inventory(
        name,
        description,
        image_url,
        amount_kg,
        material_type,
        price_kg
    )
VALUES
(
    'PolyTerra PLA Cotton White',
    'PolyTerra PLA is a bioplastic 3D printing filament in which organic materials have been combined with PLA to reduce the plastic content and develop a more environmentally friendly filament.',
    'https://c-3d.niceshops.com/upload/image/product/large/default/23619_28607dbe.512x512.jpg',
     20,
     'PLA',
     39.10    
);


INSERT INTO groovy_layers.material_inventory(
        name,
        description,
        image_url,
        amount_kg,
        material_type,
        price_kg
    )
VALUES
(
    'PolyTerra PLA Peanut',
    'PolyTerra PLA is a bioplastic 3D printing filament in which organic materials have been combined with PLA to reduce the plastic content and develop a more environmentally friendly filament.',
    'https://c-3d.niceshops.com/upload/image/product/large/default/19863_1b457ab8.512x512.jpg'
     ,20,
    'PLA',
     39.10    
);


INSERT INTO groovy_layers.material_inventory(
        name,
        description,
        image_url,
        amount_kg,
        material_type,
        price_kg
    )
VALUES
(
    'PolyTerra PLA Sapphire Blue',
    'PolyTerra PLA is a bioplastic 3D printing filament in which organic materials have been combined with PLA to reduce the plastic content and develop a more environmentally friendly filament.',
    'https://c-3d.niceshops.com/upload/image/product/large/default/28230_dd0a7c17.512x512.webp'
     ,20,
     'PLA',
     39.10    
);
