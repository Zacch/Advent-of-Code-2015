//
//  Day15.swift
//  Advent of Code 2015
//
//  Created by Rolf Staflin on 2018-11-30.
//  Copyright © 2018 Piro AB. All rights reserved.
//

import Foundation

class Ingredient : Hashable {
    
    let name: String
    let capacity: Int
    let durability: Int
    let flavor: Int
    let texture: Int
    let calories: Int

    init(_ line: String) {
        let tokens = line.components(separatedBy: " ")
        name = String(tokens[0].dropLast())
        capacity = Int(tokens[2].dropLast())!
        durability = Int(tokens[4].dropLast())!
        flavor = Int(tokens[6].dropLast())!
        texture = Int(tokens[8].dropLast())!
        calories = Int(tokens[10])!
    }

    static func == (lhs: Ingredient, rhs: Ingredient) -> Bool {
        return lhs.hashValue == rhs.hashValue
        
    }
    public var hashValue: Int { get {
        return capacity << 24 + durability << 16 + flavor << 8 + texture;
    } }
}

class Recipe {
    var ingredients: [Ingredient: Int] = [:]
    var calories = 0
    var score = 0

    func add(_ spoons: Int, of ingredient: Ingredient) {
        ingredients[ingredient] = spoons
        
        var capacity = 0
        var durability = 0
        var flavor = 0
        var texture = 0
        calories = 0
        for k in ingredients.keys {
            let v = ingredients[k]!
            capacity += k.capacity * v
            durability += k.durability * v
            flavor += k.flavor * v
            texture += k.texture * v
            calories += k.calories * v
        }
        score = max(capacity, 0) * max(durability, 0) * max(flavor, 0) * max(texture, 0)
    }
}

class Day15 {
    var ingredients: [Ingredient] = []
    var bestRecipe = Recipe()
    var best500CalorieRecipe = Recipe()

    func solve () {
        let lines = Utils.readFileLines("Day15.txt")
        for line in lines {
            let ingredient = Ingredient(line)
            ingredients.append(ingredient)
        }
        findBestRecipes(100, Recipe())
        print("Part 1: \(bestRecipe.score)")
        print("Part 2: \(best500CalorieRecipe.score)")
    }

    func findBestRecipes(_ teaSpoonsLeft: Int, _ recipe: Recipe) {
        let result = Recipe()
        recipe.ingredients.forEach {k, v in result.add(v, of: k)}
        let ingredientsLeft = ingredients.count - recipe.ingredients.count
        if ingredientsLeft == 1 {
            result.add(teaSpoonsLeft, of: ingredients.last!)
            if result.calories == 500 && result.score > best500CalorieRecipe.score {
                best500CalorieRecipe = result
            }
            if result.score > bestRecipe.score {
                bestRecipe = result
            }
            return
        }
        let ingredient = ingredients[recipe.ingredients.count]
        for spoons in  1...teaSpoonsLeft - ingredientsLeft + 1 {
            result.add(spoons, of: ingredient)
            findBestRecipes(teaSpoonsLeft - spoons, result)
            result.ingredients.removeValue(forKey: ingredient)
        }
    }
}
